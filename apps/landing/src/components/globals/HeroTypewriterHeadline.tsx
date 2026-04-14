'use client';

import { useState, useEffect } from 'react';
import Image from 'next/image';
import GradientText from '@/components/common/GradientText';
import { COLORS } from '@/constants/design';

const ROTATING_WORDS = ['Bank', 'Financial App', 'Neobank', 'Wallet', 'Institution'];
const TYPING_SPEED_MS = 80;
const DELETING_SPEED_MS = 50;
const PAUSE_AFTER_TYPED_MS = 1800;
const CURSOR_BLINK_INTERVAL_MS = 530;

const LOGO_GLOW_TYPING  = 'drop-shadow(0 0 18px rgba(222, 201, 244, 0.65))';
const LOGO_GLOW_DELETING = 'drop-shadow(0 0 6px rgba(222, 201, 244, 0.5))';

export default function HeroTypewriterHeadline() {
    const [wordIndex, setWordIndex] = useState(0);
    const [displayedWord, setDisplayedWord] = useState('');
    const [isDeleting, setIsDeleting] = useState(false);
    const [cursorVisible, setCursorVisible] = useState(true);

    useEffect(() => {
        const id = setInterval(
            () => setCursorVisible((v) => !v),
            CURSOR_BLINK_INTERVAL_MS,
        );
        return () => clearInterval(id);
    }, []);

    useEffect(() => {
        const currentWord = ROTATING_WORDS[wordIndex];

        if (!isDeleting && displayedWord === currentWord) {
            const id = setTimeout(() => setIsDeleting(true), PAUSE_AFTER_TYPED_MS);
            return () => clearTimeout(id);
        }

        if (isDeleting && displayedWord === '') {
            setIsDeleting(false);
            setWordIndex((i) => (i + 1) % ROTATING_WORDS.length);
            return;
        }

        const delay = isDeleting ? DELETING_SPEED_MS : TYPING_SPEED_MS;
        const id = setTimeout(() => {
            setDisplayedWord(
                isDeleting
                    ? currentWord.slice(0, displayedWord.length - 1)
                    : currentWord.slice(0, displayedWord.length + 1),
            );
        }, delay);
        return () => clearTimeout(id);
    }, [displayedWord, isDeleting, wordIndex]);

    return (
        <>
            <div className="flex justify-center mb-4 sm:mb-6 w-full">
                <Image
                    src="/images/defindex.svg"
                    alt="DeFindex"
                    width={406}
                    height={84}
                    className="w-32 sm:w-48 md:w-80 lg:w-96 h-auto"
                    style={{
                        filter: isDeleting ? LOGO_GLOW_DELETING : LOGO_GLOW_TYPING,
                        transition: 'filter 700ms ease-in-out',
                    }}
                    priority
                />
            </div>

            <GradientText
                as="h2"
                variant="secondary"
                textStroke={COLORS.dark}
                className="
                    font-familjen-grotesk
                    text-lg
                    sm:text-2xl
                    md:text-3xl
                    lg:text-3xl
                    leading-[1.1em]
                    sm:leading-[1.04em]
                    tracking-normal
                    sm:tracking-[0.05em]
                    md:tracking-[0.1em]
                    mb-4
                    sm:mb-6
                    uppercase
                    max-w-full
                    break-words
                "
                style={{
                    fontWeight: '400',
                    fontStyle: 'normal',
                    textAlign: 'center',
                }}
            >
                {`Yield Infrastructure for every ${displayedWord}`}
                <span style={{ opacity: cursorVisible ? 1 : 0 }}>|</span>
            </GradientText>
        </>
    );
}
