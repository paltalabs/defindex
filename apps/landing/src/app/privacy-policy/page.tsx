"use client"

import Footer from "@/components/globals/Footer"
import Navbar from "@/components/globals/navbar/Navbar"
import Image from "next/image"

export default function PrivacyPolicy(){
  return (
    <div className="min-h-screen w-full bg-black relative z-0 overflow-x-hidden">
      <Image
        width={1440}
        height={6797}
        className="w-full h-full inset-0 absolute -z-10 object-cover"
        src="/images/web-background.png"
        alt=""
      />
      <Navbar/>
      <main className="container mx-auto max-w-4xl px-4 sm:px-6 py-12 sm:py-16 lg:py-20">
        <div className="bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl p-4 sm:p-8 lg:p-12 overflow-hidden">
          <header className="mb-12">
            <h1 className="font-manrope font-bold text-2xl sm:text-3xl lg:text-4xl xl:text-5xl text-white mb-4 text-pretty break-words">
              DeFindex Privacy Policy
            </h1>
            <p className="font-inter text-gray-300 text-sm text-pretty">
              Last Updated: September 12, 2025
            </p>
          </header>

          <div className="space-y-8 text-white">
            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                1. Important Information and Who We Are
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  Welcome to DeFindex ("DeFindex", "we", "our", or "us"). This Privacy Policy explains how we collect, use, and protect your personal data when you visit our website at{' '}
                  <a href="https://defindex.io" className="text-lime-200 hover:text-lime-100 underline transition-colors">
                    https://defindex.io
                  </a>{' '}
                  (the "Site") and use our decentralized yield and vault services ("Services").
                </p>
                <p className="break-words text-pretty">
                  For any questions about this Privacy Policy or our privacy practices, please contact our Data Privacy Manager at{' '}
                  <a href="mailto:privacy@paltalabs.io" className="text-lime-200 hover:text-lime-100 underline transition-colors">
                    privacy@paltalabs.io
                  </a>.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                2. Changes to This Privacy Policy
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                We may update this Privacy Policy from time to time. When we do, we will post the updated version on the Site and update the "Last Updated" date. Your continued use of the Services after changes are posted constitutes acceptance of the revised Privacy Policy.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                3. What Information Do We Collect?
              </h2>
              <div className="space-y-6">
                <div>
                  <h3 className="font-manrope font-semibold text-base sm:text-lg text-white mb-3 text-pretty break-words">a) Blockchain Data</h3>
                  <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                    DeFindex does not collect or store personal data from your interactions on the Stellar network or any other blockchain. Please note that all blockchain transactions are public, immutable, and outside DeFindex's control.
                  </p>
                </div>

                <div>
                  <h3 className="font-manrope font-semibold text-base sm:text-lg text-white mb-3 text-pretty break-words">b) Technical Data</h3>
                  <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words mb-3">
                    When you visit the Site, we may automatically collect certain technical information such as:
                  </p>
                  <ul className="list-disc list-inside text-gray-200 font-inter leading-relaxed text-pretty space-y-1 ml-4">
                    <li>IP address</li>
                    <li>Browser type and version</li>
                    <li>Pages visited and interaction data</li>
                  </ul>
                  <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words mt-3">
                    This information is used to maintain the Site, improve functionality, and enhance security.
                  </p>
                </div>

                <div>
                  <h3 className="font-manrope font-semibold text-base sm:text-lg text-white mb-3 text-pretty break-words">c) Cookies</h3>
                  <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                    We use cookies and similar technologies to improve your browsing experience. Some cookies are essential for the Site to function, while others help us analyze usage trends. You can manage your cookie preferences through your browser settings.
                  </p>
                </div>

                <div>
                  <h3 className="font-manrope font-semibold text-base sm:text-lg text-white mb-3 text-pretty break-words">d) Aggregate Data</h3>
                  <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                    We may collect anonymized, statistical information about how users interact with our Services (e.g., vault usage patterns). This data cannot identify individuals and is used only to improve performance and user experience.
                  </p>
                </div>

                <div>
                  <h3 className="font-manrope font-semibold text-base sm:text-lg text-white mb-3 text-pretty break-words">e) Social Networks</h3>
                  <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                    If you share DeFindex content on third-party social networks (e.g., Twitter, LinkedIn), your interactions are governed by those platforms' privacy policies.
                  </p>
                </div>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                4. How Do We Use Your Data?
              </h2>
              <div className="text-gray-200 font-inter leading-relaxed text-pretty space-y-3">
                <p className="break-words text-pretty">We use the limited data we collect to:</p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Provide and improve our Services</li>
                  <li>Maintain security and prevent fraud</li>
                  <li>Comply with applicable laws and regulations</li>
                  <li>Communicate with you if you contact us</li>
                </ul>
                <p className="break-words text-pretty">We do not track or store blockchain transaction data.</p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                5. Disclosure of Information
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                We do not sell or share your personal data. We may disclose technical data to trusted third-party service providers for purposes such as analytics, hosting, or legal compliance.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                6. International Transfers
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                Because DeFindex operates globally through blockchain networks and online infrastructure, data may be processed outside your country of residence. Where personal data is involved, we take appropriate safeguards in accordance with applicable laws (e.g., Standard Contractual Clauses under GDPR).
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                7. Your Rights
              </h2>
              <div className="text-gray-200 font-inter leading-relaxed text-pretty space-y-3">
                <p className="break-words text-pretty">You have rights over your personal data, including:</p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Accessing a copy of your data</li>
                  <li>Correcting or updating your data</li>
                  <li>Requesting deletion of your data</li>
                  <li>Restricting or objecting to certain processing</li>
                  <li>Withdrawing consent where applicable</li>
                </ul>
                <p className="break-words text-pretty">
                  To exercise your rights, contact us at{' '}
                  <a href="mailto:privacy@paltalabs.io" className="text-lime-200 hover:text-lime-100 underline transition-colors">
                    privacy@paltalabs.io
                  </a>.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                8. Security of Information
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                We implement industry-standard technical, physical, and organizational safeguards to protect your information against unauthorized access, misuse, or loss.
              </p>
              <p className="text-gray-200 font-inter leading-relaxed mt-3">
                However, no system is completely secure. Because DeFindex is a decentralized platform, we cannot guarantee the security of data shared through third-party services or external networks.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                9. Data Retention
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                We retain personal data only as long as necessary to fulfill the purposes outlined in this Privacy Policy, including legal, regulatory, or reporting obligations. Technical and aggregate data may be retained for analytics and security purposes.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                10. Access to Information
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                You can manage or delete cookies through your browser. You may also contact{' '}
                <a href="mailto:privacy@paltalabs.io" className="text-lime-200 hover:text-lime-100 underline transition-colors">
                  privacy@paltalabs.io
                </a>{' '}
                to request access to any personal data we may have collected about you.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                11. Children's Privacy
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                Our Services are not directed to or intended for children under 18. We do not knowingly collect personal data from users under 13 (or the minimum legal age in your jurisdiction). If we become aware that we have collected such data, we will delete it promptly.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                12. Your Choices
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                You may choose not to use our Services if you do not agree with this Privacy Policy. You may also limit certain types of data collection (such as cookies) through your browser settings, understanding this may affect functionality.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                13. Legal Rights for EEA Residents
              </h2>
              <div className="text-gray-200 font-inter leading-relaxed text-pretty space-y-3">
                <p className="break-words text-pretty">
                  If you are located in the European Economic Area (EEA), you have additional rights under the General Data Protection Regulation (GDPR), including the right to:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Request access to your personal data</li>
                  <li>Request correction or deletion of your data</li>
                  <li>Object to or restrict processing</li>
                  <li>Request transfer of your data to another provider</li>
                  <li>Withdraw consent at any time</li>
                </ul>
                <p className="break-words text-pretty">
                  To exercise these rights, please contact us at{' '}
                  <a href="mailto:privacy@paltalabs.io" className="text-lime-200 hover:text-lime-100 underline transition-colors">
                    privacy@paltalabs.io
                  </a>.
                </p>
              </div>
            </section>

            <footer className="border-t border-cyan-800/30 pt-8 mt-12">
              <p className="text-gray-300 font-inter text-sm text-center text-pretty break-words">
                Last Updated: September 12, 2025
              </p>
            </footer>
          </div>
        </div>
      </main>
      <Footer />
    </div>
  )
}