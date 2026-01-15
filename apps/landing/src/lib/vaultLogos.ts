// Mapping of vault addresses to their logo paths
export const VAULT_LOGOS: Record<string, string> = {
  // Beans USDC
  'CBNKCU3HGFKHFOF7JTGXQCNKE3G3DXS5RDBQUKQMIIECYKXPIOUGB2S3': '/images/logos/beans.svg',
  // Beans EURC
  'CAIZ3NMNPEN5SQISJV7PD2YY6NI6DIPFA4PCRUBOGDE4I7A3DXDLK5OI': '/images/logos/beans.svg',
  // Soroswap CETES
  'CC24OISYJHWXZIFZBRJHFLVO5CNN3PQSKZE5BBBZLSSI5Z23TKC6GQY2': '/images/logos/soroswap.png',
  // Soroswap USDC
  'CA2FIPJ7U6BG3N7EOZFI74XPJZOEOD4TYWXFVCIO5VDCHTVAGS6F4UKK': '/images/logos/soroswap.png',
  // Soroswap EURC
  'CCKTLDG6I2MMJCKFWXXBXMA42LJ3XN2IOW6M7TK6EWNPJTS736ETFF2N': '/images/logos/soroswap.png',
  // HANA USDC
  'CBUJZL5QAD5TOPD7JMCBQ3RHR6RZWY34A4QF7UHILTDH2JF2Z3VJGY2Y': '/images/logos/hana.svg',
  // xPortal USDC
  'CD4JGS6BB5NZVSNKRNI43GUC6E3OBYLCLBQZJVTZLDVHQ5KDAOHVOIQF': '/images/logos/xportal.svg',
  // Seevcash
  'CC767WIU5QGJMXYHDDYJAJEF2YWPHOXOZDWD3UUAZVS4KQPRXCKPT2YZ': '/images/logos/seevcash.webp',
  // Rozo
  'CCDRFMZ7CH364ATQ5YSVTEJ3G3KPNFVM6TTC6N4T5REHWJS6LGVFP7MY': '/images/logos/rozo.webp',
};

export function getVaultLogo(vaultAddress: string): string | null {
  return VAULT_LOGOS[vaultAddress] ?? null;
}

/**
 * Strip common prefixes from vault names for cleaner display
 */
export function formatVaultName(name: string): string {
  return name
    .replace(/^DeFindex-Vault-/i, '')
    .replace(/^DeFindex-/i, '');
}
