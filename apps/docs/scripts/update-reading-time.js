#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const matter = require("gray-matter");
const readingTime = require("reading-time");

const DOCS_DIR = path.resolve(__dirname, "..");
const EXCLUDED_FILES = ["SUMMARY.md"];
const EXCLUDED_DIRS = ["node_modules", ".gitbook", "scripts"];
const READING_TIME_PREFIX = "⏱️";

function getMarkdownFiles(dir) {
  const files = [];

  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);

    if (entry.isDirectory()) {
      if (!EXCLUDED_DIRS.includes(entry.name)) {
        files.push(...getMarkdownFiles(fullPath));
      }
    } else if (
      entry.isFile() &&
      entry.name.endsWith(".md") &&
      !EXCLUDED_FILES.includes(entry.name)
    ) {
      files.push(fullPath);
    }
  }

  return files;
}

function stripCodeBlocks(text) {
  return text.replace(/```[\s\S]*?```/g, "");
}

function buildReadingTimeDescription(minutes, existingDescription) {
  const timeStr = `${READING_TIME_PREFIX} ${minutes} min read`;

  if (!existingDescription) {
    return timeStr;
  }

  if (existingDescription.startsWith(READING_TIME_PREFIX)) {
    // Replace existing reading time, keep the rest after " — "
    const dashIndex = existingDescription.indexOf(" — ");
    if (dashIndex !== -1) {
      const originalPart = existingDescription.slice(dashIndex + 3);
      return `${timeStr} — ${originalPart}`;
    }
    return timeStr;
  }

  return `${timeStr} — ${existingDescription}`;
}

function processFile(filePath) {
  const raw = fs.readFileSync(filePath, "utf-8");
  const { data: frontmatter, content } = matter(raw);

  const cleanContent = stripCodeBlocks(content);
  const stats = readingTime(cleanContent);
  const minutes = Math.max(1, Math.ceil(stats.minutes));

  const existingDescription =
    typeof frontmatter.description === "string"
      ? frontmatter.description.trim()
      : undefined;

  const newDescription = buildReadingTimeDescription(
    minutes,
    existingDescription,
  );

  if (frontmatter.description === newDescription) {
    return { filePath, changed: false, minutes };
  }

  frontmatter.description = newDescription;

  const updated = matter.stringify(content, frontmatter);
  fs.writeFileSync(filePath, updated, "utf-8");

  return { filePath, changed: true, minutes };
}

function main() {
  const files = getMarkdownFiles(DOCS_DIR);
  let changedCount = 0;

  console.log(`Found ${files.length} markdown files in ${DOCS_DIR}\n`);

  for (const file of files) {
    const result = processFile(file);
    const rel = path.relative(DOCS_DIR, result.filePath);
    const status = result.changed ? "UPDATED" : "OK";
    console.log(`  [${status}] ${rel} (${result.minutes} min)`);
    if (result.changed) changedCount++;
  }

  console.log(
    `\nDone. ${changedCount} file(s) updated, ${files.length - changedCount} unchanged.`,
  );
}

main();
