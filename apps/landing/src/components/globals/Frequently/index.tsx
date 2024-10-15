import React from "react";
import Unit from "@/components/globals/Frequently/Unit";

function Frequently() {
    return (
        <section className="mb-20 md:mb-[120px]">
            <div className="container">
                <div className="mx-auto max-w-[1216px] grid lg:grid-cols-2">
                    <div className="lg:max-w-[435px] lg:mb-0 mb-12">
                        <h2 className="text-linear leading-[1.03em] mb-3 xl:mb-6 bg-linear font-bold font-familjen-grotesk italic text-[48px] sm:text-[56px] lg:text-xl">
                            Frequently Asked Questions
                        </h2>
                        <p className="font-inter-tight text-[20px] xl:text-lg leading-[1.25em] text-white">
                            Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi
                            ut aliquip ex ea commodo consequat aute irure.
                        </p>
                    </div>
                    <div className="flex flex-col gap-6">
                        <Unit
                            isOpen
                            title="Frequently Asked Questions 1"
                            description="Ut enim ad minim veniam quis nostrud exercitation ullamco
                                    laboris nisi ut aliquip ex ea commodo consequat aute irure dolor"
                        />
                        <Unit
                            title="Frequently Asked Questions 2"
                            description="Ut enim ad minim veniam quis nostrud exercitation ullamco
                                    laboris nisi ut aliquip ex ea commodo consequat aute irure dolor"
                        />{" "}
                        <Unit
                            title="Frequently Asked Questions 3"
                            description="Ut enim ad minim veniam quis nostrud exercitation ullamco
                                    laboris nisi ut aliquip ex ea commodo consequat aute irure dolor"
                        />
                    </div>
                </div>
            </div>
        </section>
    );
}

export default Frequently;
