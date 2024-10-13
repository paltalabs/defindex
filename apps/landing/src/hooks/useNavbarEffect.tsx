"use client";
import {useEffect, useState} from "react";

function useNavbarEffect(initial: any, effect: any, position?: number) {
   const [navbar, setNavbar] = useState<any>(initial);
   useEffect(() => {
      const scrollEffect = () => {
         if (window.scrollY >= (position || 100)) {
            setNavbar(effect);
         } else if (window.scrollY <= 0) {
            setNavbar(initial);
         }
      };

      scrollEffect();
      window.addEventListener("scroll", scrollEffect);
   }, [effect, initial, position]);

   return navbar;
}

export default useNavbarEffect;
