"use client";
import Image from "next/image";
import { Button } from "@repo/ui/button";
import styles from "./page.module.css";
import { Container, Grid } from '@chakra-ui/react'
import CreateIndex from "../src/components/CreateIndex/CreateIndex";

export default function Home() {
  return (
    <Container className='' centerContent alignItems={'center'}>
      <CreateIndex />
    </Container>
  );
}
