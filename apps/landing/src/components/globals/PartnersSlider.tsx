import { PARTNERS } from "@/constants/partners";
import Image from "next/image";

const PartnersSlider = () => {
  const triplePartners = [...PARTNERS, ...PARTNERS, ...PARTNERS];

  return (
    <section className="py-8 md:py-16 overflow-hidden">
      <h3
        className="text-center font-familjen-grotesk text-sm md:text-base text-white/60 mb-8 md:mb-12 uppercase mt-12"
        style={{ letterSpacing: "0.2em" }}
      >
        Trusted by
      </h3>
      <div className="px-8 md:px-52">
        <div
          className="flex items-center partners-slider"
          style={{
            gap: "6rem",
            width: "max-content",
          }}
        >
          {triplePartners.map((partner, index) => (
            <div
              key={`${partner.name}-${index}`}
              className="flex-shrink-0"
              style={{ minWidth: "140px" }}
            >
              <Image
                src={partner.logo}
                alt={`${partner.name} logo`}
                width={140}
                height={48}
                className="h-10 md:h-12 w-auto mx-auto opacity-60 hover:opacity-100 transition-opacity duration-150"
                style={{
                  maxWidth: "140px",
                  objectFit: "contain",
                  filter: partner.name != 'Hana Wallet' ? "brightness(0) invert(1)" : "brightness(6) saturate(0) invert(0)",
                }}
              />
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default PartnersSlider;
