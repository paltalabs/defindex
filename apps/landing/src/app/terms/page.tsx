"use client"

import Footer from "@/components/globals/Footer"
import Navbar from "@/components/globals/navbar/Navbar"
import Image from "next/image"

const h2Class =
  "font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words"
const h3Class =
  "font-manrope font-semibold text-base sm:text-lg text-white mb-2 mt-6 text-pretty break-words"
const bodyClass =
  "space-y-4 text-gray-200 font-inter leading-relaxed text-pretty"
const pClass = "break-words text-pretty"
const ulClass = "list-disc list-inside space-y-1 ml-4 text-pretty"
const linkClass = "text-lime-300 underline break-all"

function Section({
  title,
  children,
}: {
  title: string
  children: React.ReactNode
}) {
  return (
    <section>
      <h2 className={h2Class}>{title}</h2>
      <div className={bodyClass}>{children}</div>
    </section>
  )
}

export default function TermsOfUse() {
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
          <header className="mb-12">
            <h1 className="font-manrope font-bold text-3xl text-white mb-2 text-pretty break-words">
              DeFindex — Terms of Use
            </h1>
            <p className="font-inter text-gray-300 text-pretty">
              Last Updated: May 27, 2026
            </p>
          </header>

          <div className="space-y-8 text-white">
            {/* Preamble */}
            <section>
              <div className={bodyClass}>
                <p className={pClass}>
                  DeFindex is not a financial institution, money services
                  business, custodian, broker, dealer, investment adviser, or
                  financial intermediary. DeFindex Corp develops and operates
                  non-custodial blockchain software infrastructure.
                </p>
                <p className={pClass}>
                  Access to this website, or any Site (as defined below), and use
                  of the Services (as defined below) is not offered to persons or
                  entities who reside in, are citizens of, are located in, are
                  incorporated in, or have a registered office in any Restricted
                  Territory (as defined below). If you are a Restricted Person, do
                  not attempt to access or use the Services. The use of a virtual
                  private network (“VPN”) to circumvent the restrictions set forth
                  herein is prohibited.
                </p>
                <p className={pClass}>
                  These Terms of Use, together with any documents and additional
                  terms they expressly incorporate by reference, including the
                  Privacy Policy and any other terms and conditions that DeFindex
                  Corp, a corporation organized under the laws of the Republic of
                  Panama (“DeFindex,” “Company,” “We,” “Us,” or “Our”), posts
                  publicly or makes available to you or a company or other legal
                  entity that you represent (“you” or “your”) (collectively, these
                  “Terms”), are entered into between DeFindex and you concerning
                  your access to and use of DeFindex’s websites, web applications,
                  APIs, smart contracts, and all associated services (collectively
                  with any materials and services available therein, the “Site”).
                </p>
                <p className={pClass}>
                  Please read these Terms carefully. By clicking “I agree,”
                  acknowledging these Terms by other means, or otherwise accessing
                  or using the Site or the Services, you accept and agree to be
                  bound by and to comply with these Terms. If you do not agree to
                  these Terms, then you must not access or use the Site or the
                  Services.
                </p>
                <p className={`${pClass} font-semibold text-gray-100`}>
                  THESE TERMS CONTAIN A BINDING ARBITRATION CLAUSE AND CLASS
                  ACTION WAIVER THAT AFFECT YOUR LEGAL RIGHTS. PLEASE READ SECTION
                  16 CAREFULLY.
                </p>
              </div>
            </section>

            <Section title="1. Definitions">
              <p className={pClass}>
                In these Terms, unless the context otherwise requires, the
                following terms shall have the meanings set forth below:
              </p>
              <p className={pClass}>
                <strong>“Applicable Law”</strong> means any law, regulation,
                order, directive, or other legal requirement in any jurisdiction
                applicable to you, DeFindex, the Site, or the Services.
              </p>
              <p className={pClass}>
                <strong>“API”</strong> means any application programming interface
                made available by DeFindex through the Site or otherwise,
                including the DeFindex API documented at{" "}
                <a
                  href="https://api.defindex.io/docs"
                  className={linkClass}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  https://api.defindex.io/docs
                </a>
                .
              </p>
              <p className={pClass}>
                <strong>“Digital Assets”</strong> means any digital asset,
                cryptocurrency, virtual currency, token, or other digital
                representation of value that is recorded on a distributed ledger
                or blockchain, including but not limited to stablecoins,
                governance tokens, utility tokens, and vault tokens.
              </p>
              <p className={pClass}>
                <strong>“Governmental Authority”</strong> means any government,
                regulatory body, court, agency, department, commission, board,
                bureau, or other governmental, regulatory, judicial, or
                administrative authority having jurisdiction over DeFindex, the
                Site, the Services, or Users.
              </p>
              <p className={pClass}>
                <strong>“Harvest Function”</strong> means any function, feature,
                process, or operation of or related to a Strategy smart contract
                that affects, applies, or relates outputs associated with one or
                more Underlying Protocols, in each case in accordance with the
                Strategy’s design, parameters, documentation, or coded logic.
              </p>
              <p className={pClass}>
                <strong>“Partner”</strong> means a wallet provider, fintech
                application, neobank, or other third-party entity that integrates
                DeFindex infrastructure for the distribution of Vault and Strategy
                services to end users.
              </p>
              <p className={pClass}>
                <strong>“Private Key”</strong> means the cryptographic key that
                enables a User to access, control, and transact with Digital
                Assets associated with a corresponding public address on a
                blockchain network.
              </p>
              <p className={pClass}>
                <strong>“Restricted Person”</strong> means: (a) any Person located
                in, resident of, or organized under the laws of any Restricted
                Territory; (b) any Person identified on any sanctions list
                maintained by any Governmental Authority, including the United
                Nations Security Council Consolidated Sanctions List, the list of
                Specially Designated Nationals and Blocked Persons maintained by
                the U.S. Treasury Department’s Office of Foreign Assets Control
                (OFAC), or any similar list maintained by the European Union, the
                United Kingdom, or any other relevant sanctions authority; (c) any
                Person owned or controlled by, or acting on behalf of, any Person
                described in clauses (a) or (b); or (d) any Person with whom
                transactions are prohibited or restricted under Applicable Law.
              </p>
              <p className={pClass}>
                <strong>“Restricted Territory”</strong> means any country,
                territory, or jurisdiction: (a) that is subject to comprehensive
                sanctions administered or enforced by any Governmental Authority,
                including Cuba, Iran, North Korea, Syria, the Crimea, Donetsk, and
                Luhansk regions of Ukraine, and Myanmar; (b) in which the provision
                of the Services would violate Applicable Law; or (c) as DeFindex
                may designate from time to time in its sole discretion for legal,
                regulatory, or business reasons.
              </p>
              <p className={pClass}>
                <strong>“Self-Custodial” or “Non-Custodial”</strong> means that
                Users retain exclusive control and custody of their Private Keys
                and Digital Assets on the blockchain, and DeFindex does not hold,
                control, or have access to Users’ Private Keys or Digital Assets.
              </p>
              <p className={pClass}>
                <strong>“Smart Contract”</strong> means an autonomous software
                program deployed on a cryptographically secured distributed ledger
                that executes predefined logic when triggered by on-chain
                transactions.
              </p>
              <p className={pClass}>
                <strong>“Strategy”</strong> means a smart contract developed by
                DeFindex that encodes specific yield-generation logic and
                interacts with Underlying Protocols to execute that logic.
              </p>
              <p className={pClass}>
                <strong>“Underlying Protocol”</strong> means any third-party,
                independently operated decentralized finance protocol, automated
                market maker, lending platform, staking mechanism, or other
                on-chain protocol with which DeFindex Strategies interacts.
              </p>
              <p className={pClass}>
                <strong>“User”</strong> means any Person who accesses or uses the
                Site or Services.
              </p>
              <p className={pClass}>
                <strong>“Vault”</strong> means a smart contract deployed through
                the DeFindex Vault Factory that accepts User deposits of Digital
                Assets, allocates those assets across whitelisted Strategies, and
                enables management from Vault Roles, fee collection, and User
                withdrawal.
              </p>
              <p className={pClass}>
                <strong>“Vault Role”</strong> means the roles assignable within a
                Vault, including the Vault Manager, Rebalance Manager, Fee
                Receiver, and Emergency Manager, each of which is authorized to
                perform specified role-restricted functions within the Vault in
                accordance with the applicable smart contract logic and
                configuration. Each Vault Role is associated with an Address, which
                may be an externally controlled wallet address or a smart
                contract. Where a Vault Role is assigned to a smart contract, that
                smart contract may implement its own governance, access controls,
                conditions, or sub-roles. Vault Roles are the only roles with
                authority to perform role-restricted or other privileged actions
                within a Vault. None of the Vault Roles permits the direct
                withdrawal of End User funds from the Vault. The current role
                architecture is described in the DeFindex documentation at{" "}
                <a
                  href="https://docs.defindex.io/getting-started/getting-started/vault-roles"
                  className={linkClass}
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  https://docs.defindex.io/getting-started/getting-started/vault-roles
                </a>
                .
              </p>
              <p className={pClass}>
                <strong>“Vault Manager”</strong> means the Vault Role that serves
                as the primary administrative role for a Vault and may, in
                accordance with the applicable smart contract logic and
                configuration, control certain Vault settings, role assignments,
                and upgrades, if enabled, and perform functions assigned to other
                Vault Roles.
              </p>
              <p className={pClass}>
                In these Terms: (a) headings are for convenience of reference only
                and shall not affect interpretation; (b) the singular includes the
                plural and vice versa; (c) references to Sections are references to
                sections of these Terms; (d) “include,” “includes,” and
                “including” shall be deemed to be followed by “without
                limitation”; and (e) references to any Applicable Law shall include
                such law as amended, supplemented, or replaced from time to time.
              </p>
            </Section>

            <Section title="2. Company’s Role and Platform">
              <h3 className={h3Class}>
                2.1 Self-Custodial Technology Infrastructure
              </h3>
              <p className={pClass}>
                DeFindex develops and operates non-custodial blockchain software
                infrastructure. DeFindex:
              </p>
              <ul className={ulClass}>
                <li>
                  does not hold, store, control, or have access to Users’ Private
                  Keys or Digital Assets;
                </li>
                <li>
                  cannot initiate, authorize, or reverse any transactions on
                  behalf of Users;
                </li>
                <li>
                  cannot restore, recover, or provide access to lost Private Keys,
                  seed phrases, or Digital Assets; and
                </li>
                <li>
                  does not have the ability to freeze, block, or restrict User
                  access to their Digital Assets held within Smart Contracts.
                </li>
              </ul>
              <p className={pClass}>
                Users acknowledge and agree that they are solely responsible for
                the security and management of their Private Keys, seed phrases,
                passwords, and Digital Assets, and that the loss of any of the
                foregoing may result in the permanent and irreversible loss of
                Digital Assets.
              </p>

              <h3 className={h3Class}>2.2 Services Provided</h3>
              <p className={pClass}>DeFindex provides the following Services:</p>
              <ul className={ulClass}>
                <li>
                  Strategy Smart Contracts that automate yield-generation
                  activities on Underlying Protocols;
                </li>
                <li>
                  Vault Smart Contracts that manage Digital Asset allocation and
                  distribution across whitelisted Strategies;
                </li>
                <li>execution of Harvest Function;</li>
                <li>
                  Backend APIs and SDKs that enable Partner wallets and
                  applications to integrate Vault and Strategy functionality;
                </li>
                <li>
                  a web-based frontend interface for interacting with Vaults and
                  Strategies; and
                </li>
                <li>
                  developer documentation, educational content, and performance
                  dashboards.
                </li>
              </ul>

              <h3 className={h3Class}>2.3 What DeFindex Does NOT Do</h3>
              <p className={pClass}>DeFindex does not:</p>
              <ul className={ulClass}>
                <li>
                  provide investment advice, financial recommendations, or
                  discretionary trading services;
                </li>
                <li>
                  guarantee or assure any yield, profit, or return on Users’
                  deposits;
                </li>
                <li>
                  act as a broker, dealer, custodian, investment manager,
                  exchange, or regulated financial services provider;
                </li>
                <li>
                  operate, administer, or control the Underlying Protocols or
                  their smart contracts;
                </li>
                <li>
                  determine the terms, pricing, or parameters of yield products
                  offered by Underlying Protocols;
                </li>
                <li>
                  match, execute, clear, or settle trades, or act as counterparty
                  to any transaction; or
                </li>
                <li>
                  manage or ensure Underlying Protocols or their security.
                </li>
              </ul>

              <h3 className={h3Class}>2.4 Fee Mechanism</h3>
              <p className={pClass}>
                DeFindex’s revenue is derived from performance-based fees that are
                programmatically allocated by Vault Smart Contracts. Fees are
                deducted automatically from accrued rewards and distributed to
                designated fee receivers (which may include DeFindex and Partners)
                without DeFindex ever handling or custodying User funds. The fee
                percentage is set at Vault creation and may be modified by the
                Vault Role or the fee receiver within the smart contract-defined
                limits.
              </p>
            </Section>

            <Section title="3. Vault Mechanics and Vault Roles">
              <h3 className={h3Class}>3.1 How Vaults Operate</h3>
              <p className={pClass}>
                Vaults are Smart Contracts deployed through the DeFindex Vault
                Factory that may:
              </p>
              <ul className={ulClass}>
                <li>a. accept User deposits of specific Digital Assets;</li>
                <li>
                  b. allocate deposited assets across whitelisted Strategies;
                </li>
                <li>
                  c. permit Users to withdraw their Digital Assets and any accrued
                  rewards when and as allowed by the applicable Vault’s smart
                  contract logic and configuration, subject to blockchain network
                  conditions and any pause, rescue, or other emergency actions
                  affecting a Strategy;
                </li>
                <li>
                  d. enable designated Vault Roles to perform specific
                  administrative or operational functions; and
                </li>
                <li>
                  e. automatically deduct and distribute performance-based fees.
                </li>
              </ul>
              <p className={pClass}>
                Any person may create a Vault, define its whitelisted Strategies,
                and assign applicable Vault Roles, in each case subject to the
                Vault’s smart contract logic and configuration. DeFindex may, at
                its discretion, create Vaults on behalf of Partners or for its own
                products. Where DeFindex creates a Vault on behalf of a Partner,
                such an act constitutes a technical deployment and configuration
                service only and does not, by itself, make DeFindex the operator,
                administrator, manager, promoter, sponsor, fiduciary, custodian,
                agent, adviser, or other person responsible for that Vault, unless
                DeFindex is separately assigned a Vault Role or expressly assumes
                such responsibility in writing. Unless otherwise expressly stated,
                responsibility for the Vault’s configuration, governance,
                disclosures, strategy selection, fee settings, upgrades, and
                ongoing operation remains with the applicable Partner and/or the
                designated Vault Role holders, as reflected in the Vault’s smart
                contract logic and configuration.
              </p>

              <h3 className={h3Class}>3.2 Vault Roles</h3>
              <p className={pClass}>
                Each Vault may have certain administrative or operational roles
                assigned to designated blockchain addresses. These roles may
                include a Vault Manager, Rebalance Manager, Fee Receiver, and
                Emergency Manager. The Vault Manager may control certain Vault
                settings, role assignments, and upgrades (if enabled), and may
                perform functions assigned to other Vault Roles. The Rebalance
                Manager may allocate or reallocate Vault assets across whitelisted
                Strategies. The Fee Receiver may receive performance fees collected
                by the Vault and perform related fee-distribution functions. The
                Emergency Manager may take protective actions in response to risk
                events, including pausing, unpausing, rescuing, or unwinding
                Strategy exposure. Any Vault Role may be assigned to an externally
                controlled address or to a smart contract, and if assigned to a
                smart contract, that smart contract may implement its own
                governance, access controls, conditions, or sub-roles. All Vault
                Roles are subject to the applicable Vault’s smart contract logic
                and configuration. None of these Vault Roles can directly withdraw
                User funds from the Vault.
              </p>

              <h3 className={h3Class}>
                3.3 Vault Role Holder Responsibility and User Acknowledgment
              </h3>
              <p className={pClass}>Users acknowledge and agree that:</p>
              <ul className={ulClass}>
                <li>
                  a. actions taken by Vault Role holders may be discretionary and
                  may affect a Vault’s operation, risk profile, returns, fees,
                  available functionality, and Strategy exposure;
                </li>
                <li>
                  b. DeFindex does not guarantee the competence, conduct, incentive
                  alignment, or performance of any Partner, Vault Role holder, or
                  other third party associated with a Vault;
                </li>
                <li>
                  c. Users should evaluate the characteristics, governance, and
                  control structure of a Vault before depositing, including who
                  holds the applicable Vault Roles, whether a smart contract or
                  governed mechanism holds any Vault Role, and whether the Vault is
                  upgradeable;
                </li>
                <li>
                  d. some Vaults may be configured as upgradeable, which means that
                  their smart contract logic, functionality, or supported
                  Strategies may be modified after deployment by the applicable
                  authorized role or address, and any such change may increase
                  risk, affect how the Vault operates, or result in partial or
                  total loss of assets;
                </li>
                <li>
                  e. Vault Role holders, including Partners and other non-DeFindex
                  parties, may operate independently, and DeFindex makes no
                  representation or warranty regarding their acts, omissions, or
                  performance; and
                </li>
                <li>
                  f. where DeFindex holds a Vault Role, any action taken by
                  DeFindex in that capacity is performed subject to the applicable
                  smart contract logic and configuration and, to the maximum extent
                  permitted by Applicable Law, without liability for losses or
                  underperformance resulting from discretionary role-based actions.
                </li>
                <li>
                  g. Any Vault Role may be assigned to an externally controlled
                  address or to a smart contract, and where a Vault Role is
                  assigned to a smart contract, that smart contract may implement
                  its own governance, access controls, conditions, or sub-roles.
                </li>
              </ul>
            </Section>

            <Section title="4. Eligibility and User Representations">
              <h3 className={h3Class}>4.1 Minimum Age and Capacity</h3>
              <p className={pClass}>
                To access or use the Site or Services, you must: (a) be at least
                eighteen (18) years of age (or the age of majority in your
                jurisdiction, whichever is greater); (b) have the full legal
                capacity to enter into these Terms; and (c) not be prohibited from
                using the Services under Applicable Law.
              </p>

              <h3 className={h3Class}>4.2 Restricted Territories and Persons</h3>
              <p className={pClass}>
                The Services are not offered to, and may not be used by, any
                Restricted Person. Without limiting the foregoing, Users represent
                and warrant that:
              </p>
              <ul className={ulClass}>
                <li>
                  they do not reside in, are not located in, or organized under the
                  laws of, and do not have a registered office in any Restricted
                  Territory;
                </li>
                <li>
                  they are not the subject of any sanctions administered or
                  enforced by OFAC, the United Nations Security Council, the
                  European Union, His Majesty’s Treasury, or any other relevant
                  sanctions authority;
                </li>
                <li>
                  they will not use the Services to facilitate transactions
                  involving any Restricted Person or Restricted Territory;
                </li>
                <li>
                  the Digital Assets they use in connection with the Services are
                  not derived from any prohibited or sanctioned activities; and
                </li>
                <li>
                  they do not, and will not, use a VPN or any other privacy or
                  anonymization tools to circumvent any restrictions that apply to
                  the Site or the Services.
                </li>
              </ul>

              <h3 className={h3Class}>4.3 Panama Sanctions Framework</h3>
              <p className={pClass}>
                Access to and use of the Site is further restricted in accordance
                with the Republic of Panama’s obligations under Title VI of Law 23
                of 2015, Executive Decree 587 of 2015, and Article 4 of the
                Political Constitution. Accordingly, Users who appear on the United
                Nations Security Council Consolidated Sanctions List are strictly
                prohibited from using the Site or interacting with DeFindex Smart
                Contracts. DeFindex may, at its discretion, additionally restrict
                access from users associated with high-risk jurisdictions or
                persons appearing on international control or reference lists
                (including OFAC, UK Treasury, EU, and FATF lists) as a voluntary
                risk-mitigation measure.
              </p>

              <h3 className={h3Class}>4.4 User Representations and Warranties</h3>
              <p className={pClass}>
                By accessing or using the Site or Services, you represent and
                warrant that:
              </p>
              <ul className={ulClass}>
                <li>
                  you own or are legally authorized to control the Digital Assets
                  you deposit into Vaults;
                </li>
                <li>
                  your Digital Assets are not proceeds of any crime, fraud, or
                  unlawful activity;
                </li>
                <li>
                  you comply with all applicable anti-money laundering,
                  counter-terrorism financing, and similar laws and regulations;
                </li>
                <li>
                  you understand the technical nature of blockchain, Smart
                  Contracts, and decentralized finance;
                </li>
                <li>
                  you have independently reviewed and accept the risks described in
                  Section 11;
                </li>
                <li>
                  your use of the Services does not violate any Applicable Law; and
                </li>
                <li>
                  all information you provide is current, complete, and accurate.
                </li>
              </ul>

              <h3 className={h3Class}>4.5 Notification of Changes</h3>
              <p className={pClass}>
                You agree to immediately notify DeFindex if any of your
                representations or warranties become untrue or inaccurate, or if
                you become aware of any circumstances that would make your
                continued use of the Services unlawful.
              </p>

              <h3 className={h3Class}>4.6 Verification Rights</h3>
              <p className={pClass}>
                DeFindex reserves the right, at any time and in its sole
                discretion, to require Users to provide information and
                documentation for identity verification, jurisdictional
                compliance, or anti-money laundering screening. Failure to provide
                requested information may result in immediate suspension or
                termination of access to the Site and Services.
              </p>
            </Section>

            <Section title="5. User Responsibilities">
              <p className={pClass}>
                Users act as principals in all interactions with blockchain
                networks, Smart Contracts, and Underlying Protocols. DeFindex does
                not act as an agent, representative, or fiduciary for any User.
                Users are exclusively and solely responsible for:
              </p>
              <ul className={ulClass}>
                <li>
                  all decisions regarding Private Keys, Digital Assets, and
                  transactions;
                </li>
                <li>
                  review and approval of transaction details before signing any
                  transaction using their locally-stored Private Key;
                </li>
                <li>compliance with Applicable Laws in their jurisdiction;</li>
                <li>
                  tax reporting and obligations related to Digital Asset
                  activities;
                </li>
                <li>
                  due diligence regarding Strategies, Vaults, Underlying Protocols,
                  and Partners;
                </li>
                <li>
                  understanding the risks associated with Digital Assets and
                  blockchain technology;
                </li>
                <li>
                  implementing appropriate security measures for their Digital
                  Assets and Private Keys; and
                </li>
                <li>
                  maintaining the security and confidentiality of Private Keys,
                  seed phrases, passwords, and other credentials associated with
                  their blockchain wallets.
                </li>
              </ul>
            </Section>

            <Section title="6. Prohibited Uses">
              <p className={pClass}>
                You shall not use the Site or Services to engage in any of the
                following Prohibited Uses. This list is illustrative and not
                exhaustive; DeFindex reserves the right to determine, in its sole
                discretion, whether your use constitutes a Prohibited Use.
              </p>
              <ul className={ulClass}>
                <li>
                  violate any Applicable Law, including anti-money laundering,
                  counter-terrorist financing, or sanctions laws administered by
                  OFAC, the United Nations Security Council, the European Union, or
                  any other relevant authority;
                </li>
                <li>
                  engage in fraud, deception, misrepresentation, market
                  manipulation, wash trading, front-running, spoofing, or any other
                  deceptive or manipulative activity;
                </li>
                <li>
                  access or use the Services from any Restricted Territory,
                  including through the use of a VPN;
                </li>
                <li>
                  use automated tools, bots, or scripts to access the Site without
                  DeFindex’s express written consent, except through the published
                  API;
                </li>
                <li>
                  interfere with, disrupt, negatively affect, or inhibit other
                  Users from accessing or using the Services, or damage, disable,
                  or impair the operation or security of the Site;
                </li>
                <li>
                  introduce viruses, malware, or other malicious code into the Site
                  or Services;
                </li>
                <li>
                  attempt to circumvent any security measures, access controls, or
                  content-filtering techniques;
                </li>
                <li>
                  use the Services to transmit or exchange Digital Assets that are
                  the direct or indirect proceeds of any criminal or fraudulent
                  activity;
                </li>
                <li>
                  provide false, inaccurate, or misleading information while using
                  the Services;
                </li>
                <li>
                  create Vaults or deploy Strategies designed to defraud, mislead,
                  or harm other Users; or
                </li>
                <li>
                  encourage, induce, or assist any third party to engage in any
                  Prohibited Use.
                </li>
              </ul>
              <p className={pClass}>
                DeFindex reserves the right to investigate and take appropriate
                action against any User who violates these prohibitions, including
                suspending or terminating access and referring the matter to law
                enforcement or regulatory authorities.
              </p>
            </Section>

            <Section title="7. Partner and Third-Party Integrations">
              <h3 className={h3Class}>7.1 Partner Wallets and Applications</h3>
              <p className={pClass}>
                DeFindex integrates with Partner wallets and fintech applications
                that distribute Vaults to end Users. Users accessing DeFindex
                through Partners acknowledge that:
              </p>
              <ul className={ulClass}>
                <li>
                  DeFindex does not operate, control, or maintain any Partner
                  application;
                </li>
                <li>
                  each Partner operates independently with its own terms of
                  service, privacy policies, and risk profiles;
                </li>
                <li>
                  Users interact directly with Partner applications, and DeFindex
                  serves only as the underlying infrastructure provider.
                </li>
                <li>
                  DeFindex makes no representations or warranties regarding the
                  functionality, security, reliability, or legal compliance of any
                  Partner; and
                </li>
                <li>
                  DeFindex shall not be liable for Partner breaches,
                  misrepresentations, failures, or actions.
                </li>
              </ul>

              <h3 className={h3Class}>7.2 Underlying Protocols and/or Assets</h3>
              <p className={pClass}>
                Strategies interact with independently operated Underlying
                Protocols and/or Assets. Users acknowledge that:
              </p>
              <ul className={ulClass}>
                <li>
                  DeFindex does not operate, control, or maintain any Underlying
                  Protocols and/or Assets;
                </li>
                <li>
                  Underlying Protocols and/or Assets are subject to their own
                  risks, including smart contract vulnerabilities, governance
                  changes, and regulatory actions;
                </li>
                <li>
                  DeFindex makes no representations regarding the security,
                  reliability, or continued operation of any Underlying Protocol
                  and/or Assets; and
                </li>
                <li>
                  Users assume all risks associated with interactions under the
                  Underlying Protocol and/or Assets.
                </li>
              </ul>

              <h3 className={h3Class}>7.3 Third-Party Links and Resources</h3>
              <p className={pClass}>
                The Site may contain links to external websites, applications, or
                resources. DeFindex is not responsible for the availability,
                accuracy, or content of such third-party resources, and does not
                endorse any third-party products or services. Any reliance on
                third-party resources is at the User’s sole risk.
              </p>
            </Section>

            <Section title="8. Fees and Payment">
              <h3 className={h3Class}>8.1 Performance Fees</h3>
              <p className={pClass}>
                DeFindex and, where applicable, Partners or other designated fee
                recipients may receive performance-based fees in connection with
                Vaults. Performance fees are:
              </p>
              <ul className={ulClass}>
                <li>
                  a. calculated as a percentage of net positive returns, accrued
                  rewards, or other yield generated by the applicable Vault or
                  Strategy, as determined by the applicable smart contract logic
                  and Vault configuration;
                </li>
                <li>
                  b. deducted, allocated, locked, released, and/or distributed
                  automatically or programmatically by the applicable smart
                  contracts to the relevant fee recipients, without DeFindex taking
                  custody of User funds;
                </li>
                <li>
                  c. set at Vault creation or otherwise determined pursuant to the
                  applicable Vault configuration, and may be modified, in whole or
                  in part, by DeFindex and/or the applicable authorized Vault Role
                  holder, in each case in accordance with the applicable smart
                  contract logic, Vault configuration, interface disclosures, and
                  Applicable Law; and
                </li>
                <li>
                  d. disclosed, reflected, or otherwise made available through the
                  applicable smart contract logic, Site, interface, API
                  documentation, Vault materials, or other relevant materials made
                  available by DeFindex or the applicable Partner.
                </li>
              </ul>

              <h3 className={h3Class}>8.2 Additional Fee Models</h3>
              <p className={pClass}>
                DeFindex may, at any time and in its sole discretion, introduce,
                implement, modify, replace, increase, decrease, suspend, remove, or
                otherwise change any fee model or fee amount applicable to the Site
                or Services, including management fees at the Strategy or Vault
                level, API usage fees, vault creation fees, transaction-based fees,
                platform fees, service fees, spreads, or other economic terms. Any
                such fees or changes may become effective upon their reflection in
                the applicable smart contract logic, Vault configuration, Site,
                interface, API documentation, or other relevant materials made
                available by DeFindex, unless otherwise stated by DeFindex. They
                may apply without prior individualized notice, subject to
                Applicable Law.
              </p>

              <h3 className={h3Class}>8.3 Blockchain Network Fees</h3>
              <p className={pClass}>
                Users are solely responsible for all blockchain network fees, gas
                fees, validator fees, relayer fees, bridge fees, or similar
                third-party charges required to interact with the applicable
                blockchain network(s), smart contracts, Vaults, Strategies, or
                third-party protocols. DeFindex does not set, control, or retain
                such third-party fees unless expressly stated otherwise.
              </p>

              <h3 className={h3Class}>8.4 Non-Refundable Fees</h3>
              <p className={pClass}>
                Except to the extent required by Applicable Law, all fees, charges,
                and amounts allocated, deducted, distributed, or otherwise incurred
                in connection with the Site or Services are non-refundable,
                including where a Vault, Strategy, transaction, or integration
                underperforms, fails to achieve any expected result, is paused,
                upgraded, modified, rescued, unwound, or becomes unavailable.
              </p>

              <h3 className={h3Class}>8.5 Fee Disclosure and Controls</h3>
              <p className={pClass}>
                Users are responsible for reviewing the applicable fees and
                economic terms before interacting with any Vault, Strategy, API, or
                other Service. The fees applicable to a particular interaction may
                depend on the relevant smart contract logic, Vault configuration,
                user interface, Partner integration, and other contextual factors.
                By proceeding with the relevant interaction, the User authorizes
                the application of the fees then in effect as reflected or
                otherwise made available through the applicable DeFindex or Partner
                materials.
              </p>
            </Section>

            <Section title="9. No Professional Advice or Fiduciary Duties">
              <p className={pClass}>
                All information provided in connection with your access and use of
                the Site and Services is for informational purposes only and should
                not be construed as professional, financial, legal, or other
                advice. You should not take or refrain from taking any action based
                on any information contained on the Site or any other content that
                DeFindex may make available, including blog posts, dashboards (such
                as Dune Analytics), documentation, social media posts, tutorials,
                or videos.
              </p>
              <p className={pClass}>
                Before you make any financial, legal, or other decisions involving
                the Services, you should seek independent professional advice from
                a qualified, licensed professional.
              </p>
              <p className={pClass}>
                These Terms are not intended to, and do not, create or impose any
                fiduciary duties on DeFindex. To the fullest extent permitted by
                Applicable Law, DeFindex owes no fiduciary duties or liabilities to
                you or any other party. To the extent that any such duties or
                liabilities may exist at law or in equity, you hereby irrevocably
                disclaim, waive, and eliminate those duties and liabilities.
              </p>
            </Section>

            <Section title="10. Marketing, Disclosures, and Information">
              <h3 className={h3Class}>10.1 No Guarantee of Returns</h3>
              <p className={pClass}>
                DeFindex does not guarantee, promise, or imply guaranteed returns
                or yield. Past performance of any Strategy or Vault, as displayed
                on the Site or dashboards, is not indicative of future results. All
                information regarding historical performance is provided for
                informational purposes only.
              </p>

              <h3 className={h3Class}>10.2 Incentivized or Promotional Yield</h3>
              <p className={pClass}>
                DeFindex, a Partner, or a third party may, from time to time,
                provide additional rewards, subsidies, or other economic incentives
                to a Vault or Strategy, which may increase the yield, rewards rate,
                or other performance metrics displayed to Users. Any such
                incentivized or promotional yield may be temporary, discretionary,
                conditional, modified, suspended, or terminated at any time, and
                does not necessarily reflect the organic or base performance of the
                applicable Vault or Strategy. Unless expressly stated otherwise,
                Users acquire no vested right to the continuation of any such
                incentive, and DeFindex does not guarantee the duration, amount, or
                continued availability of any boosted or promotional yield.
              </p>

              <h3 className={h3Class}>
                10.3 Risk Disclosure for Complex Strategies
              </h3>
              <p className={pClass}>
                DeFindex is not obliged to provide specific, enhanced risk
                disclosures for advanced Strategies that involve leverage, looping,
                liquidation risk, or multi-step composability. You should
                understand that oracle dependencies, historical stress scenarios
                exist, and total loss is possible.
              </p>

              <h3 className={h3Class}>10.4 Data Accuracy</h3>
              <p className={pClass}>
                While DeFindex strives to provide accurate information, data
                displayed on the Site, APIs, or dashboards may be delayed,
                incomplete, or inaccurate. DeFindex does not guarantee the
                accuracy, completeness, or timeliness of any data, including
                performance metrics, yield rates, or transaction information. Users
                should independently verify all data on-chain before making
                decisions.
              </p>
              <p className={pClass}>
                Without limiting the foregoing, any transaction templates, encoded
                instructions, calldata, unsigned transaction payloads, approval
                flows, or other transaction-related outputs generated or made
                available through the Site, APIs, SDKs, or dashboards are provided
                on an “as is” basis for informational and technical convenience
                only. DeFindex does not represent or warrant that any such output
                is accurate, complete, executable, successful, secure, suitable, or
                free from error. Users are solely responsible for independently
                reviewing and verifying all transaction parameters and underlying
                smart contract interactions before signing, submitting, or
                broadcasting any transaction.
              </p>
            </Section>

            <Section title="11. Risks">
              <p className={`${pClass} font-semibold text-gray-100`}>
                BY ACCESSING OR USING THE SITE OR SERVICES, YOU ACKNOWLEDGE,
                UNDERSTAND, AND AGREE TO THE INHERENT RISKS ASSOCIATED WITH
                CRYPTOGRAPHIC SYSTEMS, BLOCKCHAIN-BASED NETWORKS, DIGITAL ASSETS,
                AND SMART CONTRACTS. YOU EXPRESSLY ASSUME ALL RISKS DESCRIBED IN
                THIS SECTION AND ELSEWHERE IN THESE TERMS.
              </p>

              <h3 className={h3Class}>11.1 Smart Contract and Technology Risk</h3>
              <p className={pClass}>
                Smart Contracts are complex software and may contain bugs,
                vulnerabilities, or design flaws. Although DeFindex Smart Contracts
                have been audited by third-party providers, audits do not guarantee
                the absence of defects. Users are exposed to the risk of loss from
                code errors in DeFindex Smart Contracts, vulnerabilities in
                Underlying Protocols, and exploits or hacks affecting any Smart
                Contract in the transaction chain.
              </p>

              <h3 className={h3Class}>11.2 Underlying Protocol Risk</h3>
              <p className={pClass}>
                Strategies depend on the continued operation and security of
                Underlying Protocols. DeFindex does not control these protocols and
                cannot protect Users from smart contract hacks or exploits, rug
                pulls, token devaluations, sudden cessation of rewards, governance
                attacks, or regulatory actions against Underlying Protocols.
              </p>

              <h3 className={h3Class}>11.3 Strategy and Yield Risk</h3>
              <p className={pClass}>
                Strategies may experience negative returns or total loss of
                deposits due to adverse market conditions, price slippage, low
                liquidity, changes in reward rates or incentive structures,
                improper harvest execution, curation or misconfiguration, or
                liquidation events in higher-risk strategies.
              </p>

              <h3 className={h3Class}>11.4 Leverage and Complex Strategy Risk</h3>
              <p className={pClass}>
                DeFindex may offer or enable advanced Strategies involving
                leverage, looping, or complex composability. These strategies carry
                substantially elevated risks: leveraged positions amplify both
                gains and losses; oracle price changes or liquidity constraints may
                trigger rapid liquidations; and multi-stage transactions may fail
                at any step, resulting in partial loss or stuck assets.
              </p>

              <h3 className={h3Class}>11.5 API and Infrastructure Risk</h3>
              <p className={pClass}>
                DeFindex provides APIs to facilitate Partner integration. API
                unavailability or downtime may prevent Users from accessing Vault
                data or generating transactions through Partner applications.
                Incorrect transaction data generated by APIs may cause Users to
                sign unintended transactions. API services are provided “as-is”
                without guaranteed uptime. Users should independently verify all
                transaction details on-chain before signing.
              </p>

              <h3 className={h3Class}>11.6 Bridge and Cross-Chain Risk</h3>
              <p className={pClass}>
                DeFindex may offer infrastructure that enables the movement of
                Digital Assets across blockchain networks through bridge
                mechanisms. Bridge transactions are subject to risks including
                smart contract failures, message delays, chain reorganizations, and
                total loss of bridged assets. DeFindex does not operate bridge
                protocols and shall not be liable for bridge failures.
              </p>

              <h3 className={h3Class}>11.7 Regulatory Risk</h3>
              <p className={pClass}>
                The regulatory framework governing blockchain technologies, Digital
                Assets, and decentralized finance is evolving and uncertain. New
                laws, regulations, or policies may have a material and adverse
                impact on the Services. Users are solely responsible for
                determining whether their use of the Services complies with
                Applicable Law in their jurisdiction.
              </p>

              <h3 className={h3Class}>11.8 Digital Asset Market Risk</h3>
              <p className={pClass}>
                Digital Asset markets are highly volatile. The value of deposited
                assets and earned rewards may fluctuate significantly, and Users
                may experience permanent capital losses. Users should carefully
                consider whether participating in DeFi strategies is suitable given
                their circumstances and financial resources.
              </p>

              <h3 className={h3Class}>11.9 Private Key and Wallet Risk</h3>
              <p className={pClass}>
                Users are solely responsible for the security of their Private Keys
                and wallet credentials. DeFindex does not have access to Users’
                Private Keys and cannot restore lost keys or recover lost Digital
                Assets. Loss of Private Keys will result in permanent, irreversible
                loss of Digital Assets.
              </p>

              <h3 className={h3Class}>
                11.10 Cryptographic and Technological Advances
              </h3>
              <p className={pClass}>
                Advances in cryptography, including the development of quantum
                computing, may present risks to Digital Assets and blockchain
                security.
              </p>

              <h3 className={h3Class}>11.11 Assumption of All Risks</h3>
              <p className={pClass}>
                You acknowledge and agree that DeFindex shall have no
                responsibility or liability for any of the risks outlined in this
                Section 11. To the fullest extent permitted by Applicable Law, you
                hereby irrevocably waive, release, and discharge all claims, whether
                known or unknown, arising out of or relating to any of the risks
                outlined in this Section 11 against DeFindex, its affiliates,
                shareholders, directors, officers, employees, agents,
                representatives, suppliers, and contractors.
              </p>
            </Section>

            <Section title="12. API Terms">
              <h3 className={h3Class}>12.1 API Availability</h3>
              <p className={pClass}>
                DeFindex provides APIs and SDKs for Partner integration “as-is,”
                without guaranteed uptime or service-level agreements. DeFindex may
                modify or discontinue API endpoints with or without notice and
                shall not be liable for API downtime, data loss, or service
                interruptions. Partners should implement fallback mechanisms and
                independent monitoring.
              </p>

              <h3 className={h3Class}>12.2 API Data Accuracy</h3>
              <p className={pClass}>
                API responses provide transaction data and performance information.
                Such data may be delayed, incomplete, or incorrect. DeFindex does
                not guarantee the accuracy or completeness of API data, and Users
                should independently verify transactions on-chain before acting.
              </p>

              <h3 className={h3Class}>12.3 Transaction Signing Risk</h3>
              <p className={pClass}>
                When Users sign transactions via API-generated data, they are
                solely responsible for verifying transaction details before
                signing. DeFindex does not guarantee that API-generated
                transactions will execute as intended and shall not be liable for
                loss arising from malformed or unintended transactions.
              </p>
            </Section>

            <Section title="13. Intellectual Property">
              <h3 className={h3Class}>13.1 License Grant</h3>
              <p className={pClass}>
                Subject to these Terms, DeFindex grants you a limited, revocable,
                non-exclusive, non-transferable, and non-sublicensable license to
                access and use the Site and Services solely for personal or
                internal business purposes. This license does not grant any rights
                to copy, reproduce, commercialize, or modify the Site beyond what
                is expressly permitted.
              </p>

              <h3 className={h3Class}>13.2 Restrictions</h3>
              <p className={pClass}>Except as expressly permitted, you may not:</p>
              <ul className={ulClass}>
                <li>
                  copy, modify, distribute, create derivative works of, or
                  decompile the Services;
                </li>
                <li>
                  reverse engineer, disassemble, or attempt to derive the source
                  code of any software included in the Services;
                </li>
                <li>remove or alter any proprietary notices, labels, or marks;</li>
                <li>
                  use scraping or data-mining techniques except as expressly
                  permitted;
                </li>
                <li>
                  resell, sublicense, assign, or otherwise transfer rights to the
                  Services without prior written consent; or
                </li>
                <li>circumvent any security measures or access controls.</li>
              </ul>

              <h3 className={h3Class}>13.3 Open-Source Components</h3>
              <p className={pClass}>
                Certain components of the Services may be subject to open-source
                licenses. Such components remain under their original licenses.
                DeFindex provides open-source components “as is” without additional
                warranties.
              </p>

              <h3 className={h3Class}>13.4 Trademarks</h3>
              <p className={pClass}>
                The DeFindex name, logo, and other DeFindex trademarks are
                proprietary to DeFindex. Users may not use DeFindex trademarks
                without prior written consent. “Powered by DeFindex” attribution
                may be required for Partner integrations as specified in applicable
                Partner agreements.
              </p>

              <h3 className={h3Class}>13.5 Feedback</h3>
              <p className={pClass}>
                Any feedback, suggestions, or ideas you provide to DeFindex are
                made on a non-confidential, non-proprietary basis. By submitting
                feedback, you grant DeFindex a perpetual, irrevocable,
                non-exclusive, worldwide, royalty-free license to use, reproduce,
                modify, and disclose such feedback for any purpose.
              </p>
            </Section>

            <Section title="14. Disclaimers and Warranties">
              <p className={`${pClass} font-semibold text-gray-100`}>
                THE SITE AND SERVICES (INCLUDING ANY CONTENT, DATA, SMART
                CONTRACTS, OR FUNCTIONALITY) ARE PROVIDED ON AN “AS IS” AND “AS
                AVAILABLE” BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR
                IMPLIED, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY,
                FITNESS FOR A PARTICULAR PURPOSE, TITLE, AND NON-INFRINGEMENT.
              </p>
              <p className={pClass}>
                DEFINDEX EXPRESSLY DISCLAIMS, AND YOU HEREBY WAIVE, ANY
                REPRESENTATIONS, CONDITIONS, OR WARRANTIES OF ANY KIND, WHETHER
                EXPRESS, IMPLIED, LEGAL, STATUTORY, OR ARISING FROM COURSE OF
                DEALING, USAGE OF TRADE, OR OTHERWISE.
              </p>
              <p className={pClass}>
                WITHOUT LIMITING THE FOREGOING, DEFINDEX DOES NOT REPRESENT OR
                WARRANT THAT: (A) THE SITE OR SERVICES WILL BE UNINTERRUPTED,
                AVAILABLE AT ANY PARTICULAR TIME, OR ERROR-FREE; (B) ANY DEFECTS OR
                ERRORS WILL BE IDENTIFIED OR CORRECTED; (C) ANY STRATEGY OR VAULT
                WILL PERFORM AS INTENDED, GENERATE PROFITS, OR ACHIEVE TARGET
                RETURNS; OR (D) THE SMART CONTRACTS ARE FREE FROM VULNERABILITIES OR
                EXPLOITS.
              </p>
              <p className={pClass}>
                DEFINDEX DOES NOT OPERATE, CONTROL, OR MAINTAIN THE UNDERLYING
                PROTOCOLS OR BLOCKCHAIN NETWORKS AND MAKES NO REPRESENTATION OR
                WARRANTY REGARDING THEIR LEGAL OR REGULATORY STATUS IN ANY
                JURISDICTION.
              </p>
              <p className={pClass}>
                THE COMPANY HAS NO RESPONSIBILITY OR LIABILITY TO ANY PERSON FOR ANY
                LOSSES THAT THEY MAY INCUR AS A RESULT OF THEIR ACCESS OR USE OF THE
                SITE OR SERVICES FOR ANY REASON WHATSOEVER, INCLUDING LOSSES ARISING
                FROM SOFTWARE DEFECTS, MALFUNCTIONS, OR LACK OF ACCESS.
              </p>
              <p className={pClass}>
                YOU ACKNOWLEDGE AND AGREE THAT YOUR ACCESS TO AND USE OF THE SITE
                AND SERVICES ARE AT YOUR OWN DISCRETION AND RISK.
              </p>
            </Section>

            <Section title="15. Limitation of Liability">
              <h3 className={h3Class}>15.1 Exclusion of Consequential Damages</h3>
              <p className={pClass}>
                TO THE MAXIMUM EXTENT PERMITTED UNDER APPLICABLE LAW, IN NO EVENT
                WILL DEFINDEX, ITS OFFICERS, DIRECTORS, EMPLOYEES, AGENTS,
                AFFILIATES, SHAREHOLDERS, OR CONTRACTORS (COLLECTIVELY, THE “RISK
                LIMITED PARTIES”) BE LIABLE FOR ANY INDIRECT, INCIDENTAL, SPECIAL,
                PUNITIVE, CONSEQUENTIAL, OR SIMILAR DAMAGES OF ANY KIND, INCLUDING
                DAMAGES FOR LOSS OF DIGITAL ASSETS, LOSS OF PROFITS, REVENUE, DATA,
                OPPORTUNITIES, GOODWILL, OR OTHER BUSINESS OR FINANCIAL BENEFIT,
                ARISING OUT OF OR IN CONNECTION WITH THE SITE, THE SERVICES, ANY
                SMART CONTRACT, ANY STRATEGY, ANY VAULT, OR ANY DIGITAL ASSET,
                WHETHER BASED ON CONTRACT, TORT (INCLUDING NEGLIGENCE), STRICT
                LIABILITY, BREACH OF WARRANTY, OR ANY OTHER THEORY OF LIABILITY, AND
                WHETHER OR NOT DEFINDEX HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
                DAMAGES.
              </p>

              <h3 className={h3Class}>15.2 Cap on Liability</h3>
              <p className={pClass}>
                TO THE MAXIMUM EXTENT PERMITTED UNDER APPLICABLE LAW, DEFINDEX’S
                AGGREGATE LIABILITY ARISING OUT OF OR IN CONNECTION WITH THE SITE,
                THE SERVICES, OR THESE TERMS SHALL NOT EXCEED THE TOTAL AMOUNT OF
                FEES PAID BY YOU TO DEFINDEX IN THE TWELVE (12) MONTHS PRECEDING THE
                EVENT GIVING RISE TO THE CLAIM. IF NO FEES HAVE BEEN PAID, LIABILITY
                SHALL NOT EXCEED ONE HUNDRED U.S. DOLLARS ($100).
              </p>

              <h3 className={h3Class}>15.3 Exceptions</h3>
              <p className={pClass}>
                The limitations in this Section 15 do not apply to: (a) claims for
                death or personal injury caused by gross negligence; (b) DeFindex’s
                fraud or willful misconduct; or (c) liabilities that cannot be
                excluded or limited by Applicable Law.
              </p>

              <h3 className={h3Class}>15.4 Extension to Related Parties</h3>
              <p className={pClass}>
                All limitations, waivers, and disclaimers applicable to DeFindex
                shall also apply equally to its affiliates, contributors, owners,
                directors, officers, employees, contractors, agents, and service
                providers.
              </p>
            </Section>

            <Section title="16. Dispute Resolution and Arbitration">
              <p className={`${pClass} font-semibold text-gray-100`}>
                PLEASE READ THIS SECTION CAREFULLY AS IT AFFECTS YOUR LEGAL RIGHTS,
                INCLUDING YOUR RIGHT TO FILE A LAWSUIT IN COURT OR TO HAVE A JURY
                HEAR YOUR CLAIMS.
              </p>

              <h3 className={h3Class}>16.1 Governing Law</h3>
              <p className={pClass}>
                These Terms and any relationship, right, obligation, dispute, or
                claim arising from or related to the Site or Services shall be
                governed by and construed exclusively in accordance with the
                substantive laws of the Republic of Panama, without regard to
                conflict of laws principles.
              </p>

              <h3 className={h3Class}>16.2 Good Faith Negotiation</h3>
              <p className={pClass}>
                Before initiating any legal proceeding, you and DeFindex shall first
                attempt to resolve any dispute through good-faith negotiations. The
                aggrieved party shall provide written notice specifying the nature
                of the dispute. The receiving party shall have thirty (30) days to
                provide a written response. Within sixty (60) days after the initial
                notice, the parties shall meet and confer in good faith by
                videoconference to attempt resolution.
              </p>

              <h3 className={h3Class}>16.3 Mandatory Conciliation and Arbitration</h3>
              <p className={pClass}>
                Any dispute that cannot be resolved through negotiation shall be
                submitted to conciliation and, if necessary, arbitration
                administered by the Centro de Conciliación y Arbitraje de Panamá
                (CECAP) in accordance with its procedural rules. The proceedings
                shall be conducted as follows:
              </p>
              <ul className={ulClass}>
                <li>Number of arbitrators: one (1);</li>
                <li>Seat of arbitration: Panama City, Republic of Panama;</li>
                <li>Language: Spanish;</li>
                <li>Governing law: substantive law of the Republic of Panama;</li>
                <li>The arbitral award shall be final and binding.</li>
              </ul>

              <h3 className={h3Class}>16.4 Class Action Waiver</h3>
              <p className={pClass}>
                ALL PROCEEDINGS UNDER THIS SECTION SHALL BE CONDUCTED ONLY ON AN
                INDIVIDUAL BASIS AND NOT AS A CLASS ACTION, COLLECTIVE ACTION,
                REPRESENTATIVE CLAIM, OR ON BEHALF OF ANY THIRD PARTY. EACH USER
                EXPRESSLY WAIVES ANY RIGHT TO PARTICIPATE AS A CLASS MEMBER,
                REPRESENTATIVE, OR CLAIMANT IN ANY SUCH PROCEEDING. THE ARBITRATOR
                MAY NOT CONSOLIDATE MORE THAN ONE PERSON’S CLAIMS AND MAY NOT
                PRESIDE OVER ANY FORM OF CLASS OR REPRESENTATIVE PROCEEDING.
              </p>

              <h3 className={h3Class}>16.5 Injunctive Relief</h3>
              <p className={pClass}>
                Notwithstanding Section 16.3, DeFindex may request interim,
                precautionary, or injunctive measures before any competent court to
                protect its legal rights. Such measures shall not constitute a
                waiver of arbitration.
              </p>

              <h3 className={h3Class}>16.6 Confidentiality</h3>
              <p className={pClass}>
                The parties agree to maintain confidentiality as to the existence
                and content of all conciliation and arbitration proceedings, except
                where disclosure is required by law or to enforce an award.
              </p>

              <h3 className={h3Class}>16.7 Limitation on Time to File Claims</h3>
              <p className={pClass}>
                ANY CLAIM OR CAUSE OF ACTION ARISING OUT OF OR RELATING TO THESE
                TERMS OR THE SERVICES MUST BE COMMENCED WITHIN ONE (1) YEAR AFTER
                THE CAUSE OF ACTION ACCRUES. OTHERWISE, SUCH A CLAIM IS PERMANENTLY
                BARRED.
              </p>
            </Section>

            <Section title="17. Indemnification">
              <p className={pClass}>
                You agree to defend, indemnify, and hold harmless DeFindex, its
                affiliates, directors, officers, employees, agents, representatives,
                suppliers, and contractors (collectively, the “Indemnified
                Parties”) from and against any claims, demands, lawsuits, actions,
                proceedings, investigations, liabilities, damages, losses, costs, or
                expenses (including reasonable attorneys’ fees) arising out of or
                relating to:
              </p>
              <ul className={ulClass}>
                <li>your access to and use of the Site or Services;</li>
                <li>any Digital Assets associated with your wallet address;</li>
                <li>
                  any interaction with Vaults, Strategies, or Underlying Protocols;
                </li>
                <li>your breach or alleged breach of these Terms;</li>
                <li>your violation of any Applicable Law; and</li>
                <li>
                  your violation, infringement, or misappropriation of the rights
                  of any other person or entity.
                </li>
              </ul>
              <p className={pClass}>
                If you are obligated to indemnify any Indemnified Party, DeFindex
                shall have the right to control the defense and settlement of any
                such claim. You agree to cooperate fully with DeFindex in the
                defense of any such claim and shall not settle any claim without
                DeFindex’s prior written consent.
              </p>
            </Section>

            <Section title="18. Tax Obligations">
              <p className={pClass}>
                You are exclusively and solely responsible for any tax liabilities
                that may arise from your activities through the Site or Services. It
                is your sole responsibility to ensure that all taxes are properly
                accounted for, reported, and paid. DeFindex does not undertake any
                obligation to report, collect, or disburse taxes on your behalf. You
                hereby hold DeFindex harmless from any claims, losses, damages, or
                demands arising in connection with taxes you may owe as a result of
                use of the Services.
              </p>
            </Section>

            <Section title="19. Modification, Suspension, and Termination">
              <h3 className={h3Class}>19.1 Modifications to Terms</h3>
              <p className={pClass}>
                DeFindex reserves the right to modify these Terms from time to time.
                Changes will be effective upon posting on the Site. Your continued
                use of the Services after such posting constitutes acceptance of the
                revised Terms. If you do not agree to the revised Terms, you must
                stop using the Services immediately.
              </p>

              <h3 className={h3Class}>19.2 Service Modification and Termination</h3>
              <p className={pClass}>
                DeFindex may, in its sole discretion, modify, suspend, or
                discontinue the Site or Services, in whole or in part, temporarily
                or permanently, at any time and with or without notice. DeFindex
                will not be liable for any losses resulting from modifications,
                suspensions, or terminations.
              </p>

              <h3 className={h3Class}>19.3 User Withdrawal</h3>
              <p className={pClass}>
                You may withdraw your Digital Assets and any accrued rewards from a
                Vault when permitted by the applicable Vault’s smart contract logic
                and configuration, subject to blockchain network conditions and any
                pause, rescue, or other emergency actions affecting a Strategy.
              </p>

              <h3 className={h3Class}>19.4 Survival</h3>
              <p className={pClass}>
                The following Sections shall survive any expiration or termination:
                Sections 1, 9, 11, 13, 14, 15, 16, 17, 18, and 20, in addition to
                any other provision which by law or by its nature should survive.
              </p>
            </Section>

            <Section title="20. General Provisions">
              <h3 className={h3Class}>20.1 Entire Agreement</h3>
              <p className={pClass}>
                These Terms constitute the entire agreement between you and DeFindex
                regarding the Site and Services, and supersede all prior agreements,
                understandings, and representations.
              </p>

              <h3 className={h3Class}>20.2 Severability</h3>
              <p className={pClass}>
                If any provision of these Terms is found to be unenforceable or
                invalid by a court of competent jurisdiction, such provision shall
                be interpreted as closely as possible to reflect the parties’
                original intention, and the remaining provisions shall remain in
                full force and effect.
              </p>

              <h3 className={h3Class}>20.3 Waiver</h3>
              <p className={pClass}>
                DeFindex’s failure to enforce any right or provision of these Terms
                shall not constitute a waiver of such right or provision. Any waiver
                must be in writing.
              </p>

              <h3 className={h3Class}>20.4 Assignment</h3>
              <p className={pClass}>
                These Terms may not be transferred or assigned by you without
                DeFindex’s prior written consent. DeFindex may assign or transfer
                any or all of its rights or obligations under these Terms without
                restriction.
              </p>

              <h3 className={h3Class}>20.5 No Third-Party Beneficiaries</h3>
              <p className={pClass}>
                Except as otherwise expressly provided, there are no third-party
                beneficiaries to these Terms other than the Indemnified Parties.
              </p>

              <h3 className={h3Class}>20.6 Force Majeure</h3>
              <p className={pClass}>
                DeFindex shall not be liable for any delay, failure, or inability to
                perform any obligation under these Terms due to circumstances beyond
                its reasonable control, including natural disasters, war, terrorism,
                government regulations, pandemics, network failures, blockchain
                malfunctions, hacks, attacks on service providers, power failures,
                or equipment malfunctions.
              </p>

              <h3 className={h3Class}>20.7 No Regulatory Supervision</h3>
              <p className={pClass}>
                DeFindex is not registered, qualified, licensed, supervised, or
                regulated by any governmental authority or financial regulator,
                whether in the Republic of Panama or in any other jurisdiction. No
                public authority has reviewed, approved, endorsed, or verified the
                accuracy, suitability, or completeness of the information made
                available through the Site.
              </p>

              <h3 className={h3Class}>20.8 Electronic Communications</h3>
              <p className={pClass}>
                You consent to receive all communications, agreements, documents,
                notices, and disclosures electronically, including by posting on the
                Site or by email. DeFindex recommends that you maintain copies of
                all communications.
              </p>

              <h3 className={h3Class}>20.9 Contact</h3>
              <p className={pClass}>
                For questions regarding these Terms, contact DeFindex at:{" "}
                <a href="mailto:hello@defindex.io" className={linkClass}>
                  hello@defindex.io
                </a>
              </p>
            </Section>

            <Section title="Acknowledgment">
              <p className={pClass}>
                By accessing or using the Site or Services, you acknowledge that you
                have read and understood these Terms of Use, have carefully reviewed
                the risk disclosures in Section 11, agree to the arbitration and
                class action waiver provisions in Section 16, and agree to be bound
                by all provisions herein.
              </p>
            </Section>
          </div>
        </div>
      </main>
      <Footer />
    </div>
  )
}
