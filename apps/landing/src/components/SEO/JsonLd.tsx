export function SoftwareApplicationSchema() {
  const schema = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    "name": "DeFindex API",
    "applicationCategory": "DeveloperApplication",
    "operatingSystem": "Web",
    "description": "Yield infrastructure for wallets, neobanks, and fintech apps. Plug stablecoin savings into your app via REST API in hours. 100% non-custodial.",
    "offers": {
      "@type": "Offer",
      "price": "0",
      "priceCurrency": "USD",
      "description": "Free to integrate"
    },
    "provider": {
      "@type": "Organization",
      "name": "DeFindex",
      "url": "https://defindex.io"
    },
    "featureList": [
      "REST API integration — no smart contract work needed",
      "2-hour MVP, full integration in days",
      "100% non-custodial — users keep custody at the wallet level",
      "Rescue mechanics at the contract layer",
      "7 partners live across LATAM, EMEA and APAC"
    ],
    "keywords": "yield infrastructure, stablecoin savings, wallet API, neobank, fintech, USDC yield, non-custodial, REST API, remittance"
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
    />
  );
}

export function OrganizationSchema() {
  const schema = {
    "@context": "https://schema.org",
    "@type": "Organization",
    "name": "DeFindex",
    "url": "https://defindex.io",
    "logo": "https://defindex.io/images/logo.png",
    "description": "Yield infrastructure for wallets, neobanks, and fintech apps. 7 partners live across LATAM, EMEA and APAC. $1.1M TVL, 3k+ users.",
    "foundingDate": "2023",
    "sameAs": [
      "https://twitter.com/defindex_",
      "https://github.com/paltalabs/defindex",
      "https://discord.gg/CUC26qUTw7",
      "https://www.linkedin.com/company/defindex"
    ],
    "contactPoint": {
      "@type": "ContactPoint",
      "email": "dev@paltalabs.io",
      "contactType": "Customer Support"
    }
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
    />
  );
}

export function WebSiteSchema() {
  const schema = {
    "@context": "https://schema.org",
    "@type": "WebSite",
    "name": "DeFindex",
    "url": "https://defindex.io",
    "description": "Yield infrastructure for wallets, neobanks, and fintech apps",
    "potentialAction": {
      "@type": "SearchAction",
      "target": "https://defindex.io/search?q={search_term_string}",
      "query-input": "required name=search_term_string"
    }
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
    />
  );
}
