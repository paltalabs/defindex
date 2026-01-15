"use client";

import Footer from "@/components/globals/Footer";
import Navbar from "@/components/globals/navbar/Navbar";
import Image from "next/image";
import { useState } from "react";

type RaffleTab = "soroswap" | "hana" | "xportal";

function SoroswapTerms() {
  return (
    <div className="space-y-8 text-white">
      <header className="mb-8">
        <h1 className="font-manrope font-bold text-xl sm:text-2xl text-white mb-2 text-pretty break-words">
          Soroswap Raffle Contest
        </h1>
        <p className="font-inter text-gray-300 text-sm">Terms &amp; Conditions</p>
      </header>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          1. Eligibility
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Open to legal residents who are 18 years or older. Employees, affiliates,
          and immediate family members of Palta Labs are not eligible to participate.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          2. Entry Period
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          The raffle runs from January 16 to January 31. Entries received outside
          this period will not be considered.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          3. How to Enter
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            An entry constitutes a submission of funds in the form of the USDC token
            to the Soroswap deposit contract{" "}
            <code className="break-all rounded bg-white/10 px-1.5 py-0.5 text-sm">
              CA2FIPJ7U6BG3N7EOZFI74XPJZOEOD4TYWXFVCIO5VDCHTVAGS6F4UKK
            </code>{" "}
            that meets the following criteria:
          </p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>The balance submitted is at a minimum fifty (50) USDC</li>
            <li>The deposit is maintained in full for at least seven (7) days</li>
          </ul>
          <p>
            Limit one entry per wallet. Multiple entry attempts from the same
            individual will be disqualified.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          4. Additional Entries
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            Participants may exclusively obtain additional entries by depositing
            additional USDC tokens accordingly:
          </p>
          <div className="overflow-x-auto">
            <table className="w-full border-collapse">
              <thead>
                <tr className="border-b border-white/20">
                  <th className="px-4 py-2 text-left font-semibold text-white">
                    Deposit Amount
                  </th>
                  <th className="px-4 py-2 text-left font-semibold text-white">
                    Entries
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">50 USDC</td>
                  <td className="px-4 py-2">1 entry</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">100 USDC</td>
                  <td className="px-4 py-2">3 entries</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">200 USDC</td>
                  <td className="px-4 py-2">10 entries</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">500 USDC</td>
                  <td className="px-4 py-2">20 entries</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">1,000 USDC</td>
                  <td className="px-4 py-2">50 entries</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">2,000 USDC</td>
                  <td className="px-4 py-2">100 entries</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">5,000 USDC</td>
                  <td className="px-4 py-2">1,000 entries</td>
                </tr>
                <tr className="border-b border-white/10">
                  <td className="px-4 py-2">10,000 USDC</td>
                  <td className="px-4 py-2">5,000 entries</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          5. Prize
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            Prizes will each amount to one hundred (100) USDC tokens, with three (3)
            delivered each week over four (4) weeks. Prizes will be delivered to the
            winning Stellar wallets via the Soroswap deposit contract{" "}
            <code className="break-all rounded bg-white/10 px-1.5 py-0.5 text-sm">
              CA2FIPJ7U6BG3N7EOZFI74XPJZOEOD4TYWXFVCIO5VDCHTVAGS6F4UKK
            </code>
            .
          </p>
          <p>
            Prizes are non-transferable and cannot be exchanged for cash. No
            substitutions allowed except by sponsor due to availability, in which
            case a prize of equal or greater value will be awarded.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          6. Winner Selection
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Five (5) winners will be selected at random on or around Friday 16th,
          Friday 23rd and Sunday 1st. Odds of winning depend on the number of
          eligible entries received.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          7. Winner Notification
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Winners will be notified via email or social media within 3 days of
          selection and must respond within 24 hours to claim the prize. Failure to
          respond will result in forfeiture and selection of an alternate winner.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          8. General Conditions
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          By entering, participants agree to be bound by these terms. Sponsor is not
          responsible for lost, late, incomplete, or misdirected entries. Sponsor
          reserves the right to cancel or modify the raffle if fraud or technical
          issues compromise its integrity.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          9. Publicity
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Winner agrees to the use of their name and likeness for promotional
          purposes without additional compensation, except where prohibited by law.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          10. Limitation of Liability
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Sponsor is not responsible for any injury, loss, or damage resulting from
          participation or prize acceptance.
        </p>
      </section>
    </div>
  );
}

function HanaTerms() {
  return (
    <div className="space-y-8 text-white">
      <header className="mb-8">
        <h1 className="font-manrope font-bold text-xl sm:text-2xl text-white mb-2 text-pretty break-words">
          XP Raffle Campaign
        </h1>
        <p className="font-inter text-gray-300 text-sm">
          Hana Wallet &times; DeFindex
        </p>
      </header>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          0. Acceptance of Terms
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Participation in this campaign implies full and unconditional acceptance
          of these Terms and Conditions.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          1. Organizer
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            This campaign (the &quot;XP Raffle Campaign — Hana Wallet &times;
            DeFindex&quot;) is organized by Hana Wallet, in collaboration with
            DeFindex as a technology partner.
          </p>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              Hana Wallet is responsible for:
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Official campaign communications</li>
              <li>Definition of the XP mechanics</li>
              <li>Execution of the raffle</li>
              <li>Selection of winners</li>
              <li>Distribution of prizes</li>
            </ul>
          </div>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              DeFindex participates as:
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Provider of the funds to be distributed in the raffle</li>
              <li>Earn / Vaults infrastructure provider</li>
              <li>
                Technology collaborator with no control over winner selection or
                prize allocation
              </li>
            </ul>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          2. Campaign Period
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>The campaign will be valid from:</p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>
              <strong>Start:</strong> January 18, 2026
            </li>
            <li>
              <strong>End:</strong> January 31, 2026
            </li>
          </ul>
          <p>
            Hana Wallet reserves the right to extend, modify, or terminate the
            campaign early for operational, technical, or regulatory reasons.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          3. Campaign Objective
        </h2>
        <div className="text-gray-200 font-inter leading-relaxed">
          <p className="mb-4">The main objectives of the campaign are to:</p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>Incentivize user activity and engagement within Hana Wallet</li>
            <li>Promote the use of Earn products integrated via DeFindex</li>
            <li>Reward users through an XP-based incentive system</li>
          </ul>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          4. Campaign Mechanics (XP-Based Raffle)
        </h2>
        <div className="space-y-6 text-gray-200 font-inter leading-relaxed">
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              4.1 General Principle
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>
                Users accumulate XP (Experience Points) through eligible actions
                defined by Hana Wallet
              </li>
              <li>
                XP replaces traditional raffle tickets as the weighting mechanism
              </li>
              <li>Higher XP balances result in higher chances of winning</li>
              <li>$10 prizes for 100 people each week</li>
            </ul>
            <p className="mt-4">
              The exact XP-to-probability conversion will be defined and
              communicated by Hana Wallet.
            </p>
          </div>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              4.2 Eligible Actions
            </h3>
            <p className="mb-4">
              XP may be earned through actions such as (non-exhaustive list):
            </p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Deposits into Earn / Vaults available within Hana Wallet</li>
              <li>Minimum deposit $20 for eligibility</li>
              <li>Maintaining active balances over time</li>
              <li>Other in-app actions defined by Hana Wallet</li>
            </ul>
            <p className="mt-4">
              The final list of eligible actions and XP calculations will be
              confirmed in a dedicated announcement.
            </p>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          5. Prize Pool
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            <strong>Total prize pool:</strong> 2,000 USDC
          </p>
          <p>
            The distribution structure (single winner or multiple winners) will be
            defined by Hana Wallet and communicated prior to the raffle execution.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          6. Winner Selection
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>
              The raffle and winner selection methodology will be defined in a
              subsequent meeting
            </li>
            <li>Selection is expected to be based on XP-weighted logic</li>
            <li>Final rules will be published by Hana Wallet before the draw</li>
          </ul>
          <p>
            Hana Wallet reserves the right to adjust the selection process for
            fairness, transparency, or operational reasons.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          7. Prize Distribution
        </h2>
        <ul className="list-disc list-inside space-y-2 ml-4 text-gray-200 font-inter leading-relaxed">
          <li>Prizes will be credited to the winner&apos;s Hana Wallet account</li>
          <li>
            Estimated distribution timeframe: 7 to 14 calendar days after campaign
            closure
          </li>
          <li>
            Prizes will be delivered in USDC, potentially via Earn / Vault products
            powered by DeFindex
          </li>
          <li>
            Additional verification may be required prior to final distribution.
          </li>
        </ul>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          8. Eligibility
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <div>
            <p className="mb-4">Eligible participants must:</p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Have an active Hana Wallet account</li>
              <li>
                Comply with verification and usage requirements defined by the
                platform
              </li>
              <li>Be located in jurisdictions enabled by Hana Wallet</li>
            </ul>
          </div>
          <div>
            <p className="mb-4">The following are not eligible:</p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Employees, contractors, or direct collaborators of Hana Wallet</li>
              <li>
                Accounts suspended, restricted, or flagged for fraudulent activity
              </li>
            </ul>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          9. Abuse and Fraud Prevention
        </h2>
        <div className="text-gray-200 font-inter leading-relaxed">
          <p className="mb-4">Hana Wallet reserves the right to:</p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>Exclude users engaging in abusive or manipulative behavior</li>
            <li>Reset or invalidate XP accrued through prohibited methods</li>
            <li>
              Disqualify multiple accounts linked to the same individual or device
            </li>
          </ul>
          <p className="mt-4">
            Limit: one account per person, unless otherwise stated by Hana Wallet.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          10. Taxes and Fiscal Responsibility
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            Winners are solely responsible for any taxes, fees, or declarations
            arising from the prize in accordance with their local jurisdiction.
          </p>
          <p>
            Hana Wallet and DeFindex assume no tax liability related to the prizes
            distributed.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          11. Changes, Suspension, or Cancellation
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            Hana Wallet may modify these Terms and Conditions, or suspend or cancel
            the campaign, for technical, operational, regulatory, or force majeure
            reasons.
          </p>
          <p>
            Any changes will take effect upon publication through Hana
            Wallet&apos;s official communication channels.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          12. Claims and Support
        </h2>
        <ul className="list-disc list-inside space-y-2 ml-4 text-gray-200 font-inter leading-relaxed">
          <li>
            Claim submission period: 10 business days following prize notification
          </li>
          <li>Channel: official Hana Wallet support</li>
          <li>Supporting documentation may be required</li>
        </ul>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          13. DeFindex Disclaimer
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            DeFindex does not manage user funds, define XP mechanics, select
            winners, or distribute prizes.
          </p>
          <p>
            Its role is strictly limited to providing technological infrastructure
            for Earn products integrated into Hana Wallet.
          </p>
        </div>
      </section>
    </div>
  );
}

function XPortalTerms() {
  return (
    <div className="space-y-8 text-white">
      <header className="mb-8">
        <h1 className="font-manrope font-bold text-xl sm:text-2xl text-white mb-2 text-pretty break-words">
          Earn Raffles Campaign
        </h1>
        <p className="font-inter text-gray-300 text-sm">
          xPortal &times; DeFindex
        </p>
      </header>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          0. Acceptance of Terms
        </h2>
        <p className="text-gray-200 font-inter leading-relaxed">
          Participation in this campaign implies full and unconditional acceptance
          of these Terms and Conditions.
        </p>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          1. Organizer
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            This campaign (the &quot;Earn Raffles Campaign — xPortal &times;
            DeFindex&quot;) is organized by xPortal, in collaboration with DeFindex
            as a technology partner.
          </p>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              xPortal is responsible for:
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Official campaign communications</li>
              <li>Execution of the raffles</li>
              <li>Selection of winners</li>
              <li>Distribution of prizes</li>
            </ul>
          </div>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              DeFindex participates as:
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Provider of the funds to be distributed in the raffles</li>
              <li>Earn / Vaults infrastructure provider</li>
              <li>Technology collaborator with no control over prizes or outcomes</li>
            </ul>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          2. Campaign Period
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>The campaign will be valid from:</p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>
              <strong>Start:</strong> To be announced
            </li>
            <li>
              <strong>End:</strong> To be announced
            </li>
          </ul>
          <p>
            xPortal reserves the right to extend, modify, or terminate the campaign
            early for operational, technical, or regulatory reasons.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          3. Campaign Objective
        </h2>
        <div className="text-gray-200 font-inter leading-relaxed">
          <p className="mb-4">The main objectives of the campaign are to:</p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>Incentivize the use of Earn within xPortal</li>
            <li>
              Increase TVL (Total Value Locked) in vaults integrated via DeFindex
            </li>
            <li>Reward users who maintain higher deposited balances</li>
          </ul>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          4. Campaign Mechanics (Weighted Raffles)
        </h2>
        <div className="space-y-6 text-gray-200 font-inter leading-relaxed">
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              4.1 General Principle
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Each 1 USDC deposited = 1 raffle ticket</li>
              <li>Higher deposited amounts generate a higher number of tickets</li>
              <li>More tickets result in a higher probability of winning</li>
              <li>There is no maximum limit on the number of tickets per user.</li>
            </ul>
          </div>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              4.2 Eligible Deposits
            </h3>
            <p className="mb-4">
              The following are considered eligible for ticket calculation:
            </p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Deposits made into Earn / Vaults available within xPortal</li>
              <li>Denominated in USDC</li>
              <li>Maintained throughout the campaign period</li>
            </ul>
            <p className="mt-4 mb-4">The following are not eligible:</p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Funds withdrawn before the campaign closes</li>
              <li>Internal transfers not associated with Earn</li>
              <li>Deposits made outside the xPortal interface</li>
            </ul>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          5. Prize Structure
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            The final prize format will be defined by xPortal prior to the official
            launch.
          </p>
          <div>
            <h3 className="font-manrope font-semibold text-sm sm:text-base text-white mb-3">
              Option A — Three Raffles
            </h3>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Raffle 1: 500 USDC</li>
              <li>Raffle 2: 750 USDC</li>
              <li>Raffle 3: 750 USDC</li>
            </ul>
            <p className="mt-4">
              Each raffle will be executed on predefined dates within the campaign
              period.
            </p>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          6. Winner Selection
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>
              Winners will be selected using a weighted random mechanism based on
              the number of tickets
            </li>
            <li>Each ticket represents a proportional chance of winning</li>
            <li>
              The selection system will be operated and internally audited by
              xPortal
            </li>
          </ul>
          <p>
            In the event of technical ties or operational issues, xPortal may apply
            additional criteria or repeat the draw.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          7. Prize Distribution
        </h2>
        <ul className="list-disc list-inside space-y-2 ml-4 text-gray-200 font-inter leading-relaxed">
          <li>Prizes will be credited to the winner&apos;s xPortal account</li>
          <li>
            Estimated distribution timeframe: 7 to 14 calendar days after campaign
            closure
          </li>
          <li>Prizes will be delivered in USDC within DeFindex vaults</li>
          <li>
            xPortal may require additional verification prior to final prize
            distribution.
          </li>
        </ul>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          8. Eligibility
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <div>
            <p className="mb-4">Eligible participants must:</p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Have an active xPortal account</li>
              <li>
                Comply with verification requirements defined by the platform
              </li>
              <li>Be located in jurisdictions enabled by xPortal</li>
            </ul>
          </div>
          <div>
            <p className="mb-4">The following are not eligible:</p>
            <ul className="list-disc list-inside space-y-2 ml-4">
              <li>Employees, contractors, or direct collaborators of xPortal</li>
              <li>
                Accounts that are suspended, restricted, or flagged for fraudulent
                activity
              </li>
            </ul>
          </div>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          9. Abuse and Fraud Prevention
        </h2>
        <div className="text-gray-200 font-inter leading-relaxed">
          <p className="mb-4">xPortal reserves the right to:</p>
          <ul className="list-disc list-inside space-y-2 ml-4">
            <li>Exclude users engaging in abusive behavior</li>
            <li>Invalidate artificially generated tickets</li>
            <li>
              Disqualify multiple accounts linked to the same individual or device
            </li>
          </ul>
          <p className="mt-4">Limit: one account per person.</p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          10. Taxes and Fiscal Responsibility
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            Winners are solely responsible for any taxes, fees, or declarations
            arising from the prize, in accordance with their local jurisdiction.
          </p>
          <p>
            xPortal assumes no tax liability related to the prizes distributed.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          11. Changes, Suspension, or Cancellation
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            xPortal may modify these Terms and Conditions, or suspend or cancel the
            campaign, for technical, operational, regulatory, or force majeure
            reasons.
          </p>
          <p>
            Any changes will take effect upon publication through xPortal&apos;s
            official communication channels.
          </p>
        </div>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          12. Claims and Support
        </h2>
        <ul className="list-disc list-inside space-y-2 ml-4 text-gray-200 font-inter leading-relaxed">
          <li>
            Claim submission period: 10 business days following prize notification
          </li>
          <li>Channel: official xPortal support</li>
          <li>Supporting documentation may be required</li>
        </ul>
      </section>

      <section>
        <h2 className="font-manrope font-bold text-base sm:text-lg lg:text-xl text-lime-200 mb-4">
          13. DeFindex Disclaimer
        </h2>
        <div className="space-y-4 text-gray-200 font-inter leading-relaxed">
          <p>
            DeFindex does not manage user funds, select winners, or distribute
            prizes.
          </p>
          <p>
            Its role is strictly limited to providing technological infrastructure
            for Earn products integrated into xPortal.
          </p>
        </div>
      </section>
    </div>
  );
}

const tabs: { id: RaffleTab; label: string; description: string, visible: boolean }[] = [
  { id: "soroswap", label: "Soroswap", description: "USDC Earn Raffle", visible: true },
  { id: "hana", label: "Hana Wallet", description: "XP Raffle Campaign", visible: true },
  { id: "xportal", label: "xPortal", description: "Earn Raffles", visible: true },
];

export default function RaffleTermsPage() {
  const [activeTab, setActiveTab] = useState<RaffleTab>("soroswap");

  return (
    <div className="min-h-screen w-full bg-black relative z-0 overflow-x-hidden">
      <Image
        width={1440}
        height={6797}
        className="w-full h-full inset-0 absolute -z-10 object-cover"
        src="/images/web-background.png"
        alt=""
      />
      <Navbar />
      <main className="container mx-auto max-w-4xl px-4 sm:px-6 py-12 sm:py-16 lg:py-20 mt-20">
        <div className="bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl p-4 sm:p-8 lg:p-12 overflow-hidden">
          {/* Page Title */}
          <div className="mb-8">
            <h1 className="font-manrope font-bold text-2xl sm:text-3xl text-white mb-2">
              Raffle Terms &amp; Conditions
            </h1>
            <p className="font-inter text-gray-300">
              Select a raffle to view its terms and conditions
            </p>
          </div>

          {/* Tabs */}
          <div className="flex gap-2 sm:gap-3 mb-8 flex-wrap justify-center sm:justify-start">
            {tabs.map((tab) => (
              tab.visible &&
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex flex-col items-center justify-center px-6 sm:px-8 py-4 sm:py-5 rounded-xl font-manrope font-semibold text-sm sm:text-base transition-colors min-w-[140px] sm:min-w-[180px] ${
                  activeTab === tab.id
                    ? "bg-lime-200 text-black"
                    : "bg-white/10 text-white hover:bg-white/20"
                }`}
              >
                <span className="block text-base sm:text-lg font-bold">{tab.label}</span>
                <span
                  className={`text-xs sm:text-sm font-normal mt-1 ${
                    activeTab === tab.id ? "text-black/70" : "text-gray-400"
                  }`}
                >
                  {tab.description}
                </span>
              </button>
            ))}
          </div>

          {/* Content */}
          <div className="border-t border-cyan-800/30 pt-8">
            {activeTab === "soroswap" && <SoroswapTerms />}
            {activeTab === "hana" && <HanaTerms />}
            {activeTab === "xportal" && <XPortalTerms />}
          </div>
        </div>
      </main>
      <Footer />
    </div>
  );
}
