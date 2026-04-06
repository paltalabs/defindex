interface TokenInfo {
  symbol: string;
  name: string;
  icon: string;
  decimals: number;
}

// Token contract address to info mapping from Soroswap Protocol curated list
export const TOKEN_ICONS: Record<string, TokenInfo> = {
  // XRP by Muyu Network
  'CAAV3AE3VKD2P4TY7LWTQMMJHIJ4WOCZ5ANCIJPC3NRSERKVXNHBU2W7': {
    symbol: 'XRP',
    name: 'XRP by Muyu Network',
    icon: 'https://ipfs.io/ipfs/bafkreih6jv72q2zuthweyyzq5vqu4dcnxbkefd7o7shgs2llnqrdeufyiy',
    decimals: 7,
  },
  // Etherfuse CETES
  'CAL6ER2TI6CTRAY6BFXWNWA7WTYXUXTQCHUBCIBU5O6KM3HJFG6Z6VXV': {
    symbol: 'CETES',
    name: 'Etherfuse CETES',
    icon: 'https://stablebonds.s3.us-west-2.amazonaws.com/stablebond/spl-cetes.png',
    decimals: 7,
  },
  // BTC by Ultra Capital
  'CAO7DDJNGMOYQPRYDY5JVZ5YEK4UQBSMGLAEWRCUOTRMDSBMGWSAATDZ': {
    symbol: 'BTC',
    name: 'BTC by Ultra Capital',
    icon: 'https://ipfs.io/ipfs/bafkreidhjwrkvsvuixny3j6w2xk4vhr5x7urz525usl7xvjur35fsc6bmu',
    decimals: 7,
  },
  // Zombie Coin
  'CATNYPHNO3U72V7ZVUE6QESDWTSBORW2BE7WFM3BQABQNNZQ32F2HYNC': {
    symbol: 'ZOMB',
    name: 'Zombie Coin',
    icon: 'https://xlmeme.com/cdn-cgi/imagedelivery/BMsStEZme-xi9eCyEb_01g/a2b0f7e2-09ac-4a89-086e-5e928f57ab00/public',
    decimals: 7,
  },
  // AQUA Token
  'CAUIKL3IYGMERDRUN6YSCLWVAKIFG5Q4YJHUKM4S4NJZQIA3BAS6OJPK': {
    symbol: 'AQUA',
    name: 'AQUA Token',
    icon: 'https://ipfs.io/ipfs/bafkreigzckkixvbkru2gzve67zu3bfauccugkr6zovkdv6h2yxxrppyyqa',
    decimals: 7,
  },
  // Glo Dollar
  'CB226ZOEYXTBPD3QEGABTJYSKZVBP2PASEISLG3SBMTN5CE4QZUVZ3CE': {
    symbol: 'USDGLO',
    name: 'Glo Dollar',
    icon: 'https://app.glodollar.org/glo-logo.png',
    decimals: 7,
  },
  // KALE
  'CB23WRDQWGSP6YPMY4UV5C4OW5CBTXKYN3XEATG7KJEZCXMJBYEHOUOV': {
    symbol: 'KALE',
    name: 'The Blockchain Superfood',
    icon: 'https://imagedelivery.net/yd3qPvu7Jy_6BfgoQb2pZQ/30466c1d-ef73-4d2b-8814-949157465a00/public',
    decimals: 7,
  },
  // yBTC by Ultra Capital
  'CB2XMFB6BDIHFOSFB5IXHDOYV3SI3IXMNIZLPDZHC7ENDCXSBEBZAO2Y': {
    symbol: 'yBTC',
    name: 'yBTC by Ultra Capital',
    icon: 'https://ipfs.io/ipfs/bafkreifuhpzfs5f6umrzrkqcspgvchruzwyuyvtgpzhs4nyvuraertoaby',
    decimals: 7,
  },
  // Ondo USDY
  'CB3YA656OYIHU57657I5KGSBRHE5I3OZU4VFC22PYAOANFZHEWNYGAGP': {
    symbol: 'USDY',
    name: 'Ondo U.S. Dollar Yield',
    icon: 'https://cdn.ondo.finance/brand/logos/ondo-icon-black_160x160.png',
    decimals: 7,
  },
  // GBPx
  'CBCO65UOWXY2GR66GOCMCN6IU3Y45TXCPBY3FLUNL4AOUMOCKVIVV6JC': {
    symbol: 'GBPx',
    name: 'Decentralized GBP Coin',
    icon: 'https://stellar.myfilebase.com/ipfs/QmeGXphMJgKHdhoVxb6P3jEJEkj79WPheLiga7ouNd5EZm',
    decimals: 7,
  },
  // ETH by Ultra Capital
  'CBH4M45TQBLDPXOK6L7VYKMEJWFITBOL64BN3WDAIIDT4LNUTWTTOCKF': {
    symbol: 'ETH',
    name: 'ETH by Ultra Capital',
    icon: 'https://ipfs.io/ipfs/bafkreiad2di72p5p7bs3tolvimrq4bspcle3eqc74j7557wrqsnxlxlqya',
    decimals: 7,
  },
  // SSLX Cassator
  'CBHBD77PWZ3AXPQVYVDBHDKEMVNOR26UZUZHWCB6QC7J5SETQPRUQAS4': {
    symbol: 'SSLX',
    name: 'SSLX Cassator',
    icon: 'https://ipfs.io/ipfs/bafkreiecqakv3lqychk6wwzccywywvtwjag3vtehzj4m3qilcj7735cfva',
    decimals: 7,
  },
  // Reflector xRF
  'CBLLEW7HD2RWATVSMLAGWM4G3WCHSHDJ25ALP4DI6LULV5TU35N2CIZA': {
    symbol: 'XRF',
    name: 'Reflector xRF',
    icon: 'https://reflector.network/favicon-32x32.png',
    decimals: 7,
  },
  // Etherfuse USTRY
  'CBLV4ATSIWU67CFSQU2NVRKINQIKUZ2ODSZBUJTJ43VJVRSBTZYOPNUR': {
    symbol: 'USTRY',
    name: 'Etherfuse USTRY',
    icon: 'https://stablebonds.s3.us-west-2.amazonaws.com/stablebond/spl-ustry.png',
    decimals: 7,
  },
  // EURx
  'CBN3NCJSMOQTC6SPEYK3A44NU4VS3IPKTARJLI3Y77OH27EWBY36TP7U': {
    symbol: 'EURx',
    name: 'Decentralized EUR Coin',
    icon: 'https://stellar.myfilebase.com/ipfs/QmTGegMR2v86ZjzHvhT3DMD6t1ri1VUz82tqRt4SxbFjC1',
    decimals: 7,
  },
  // XRP (Stellarport)
  'CBPMFYWP4FFV7PQUYHXJZBXS75EHR6FXYSYEZWH2UM7AUYSKI2Z3PTCG': {
    symbol: 'XRP',
    name: 'Ripple',
    icon: 'https://ipfs.io/ipfs/bafkreiheegvgfzbks6tacnlw24ub2frnr7j32rkzamieifhi2avljcly6m',
    decimals: 7,
  },
  // YieldBlox
  'CBRP2VD3CZLEQIQZ4JMBXGA5AC2U6JE26YU5CCIOICIZCVWPGBO2QRUB': {
    symbol: 'YBX',
    name: 'YieldBlox',
    icon: 'https://uploads-ssl.webflow.com/607f603d2f412c67690368b8/60df683719cc056004ea932b_YBX%20token%20web%20bad.png',
    decimals: 7,
  },
  // Skyhitz HITZ
  'CBS5ZVKSSUKF4JY77CKUZPN72EDUM3OOGPYZKFC3KQVONXPJTF6UODD7': {
    symbol: 'HITZ',
    name: 'Skyhitz Token',
    icon: 'https://skyhitz.io/icon.png',
    decimals: 7,
  },
  // Poye
  'CBT2U7I6J7LSEEJQQXBEOSRXNRNLH5UQPWG74Q36J3RHNQOQ5KEFCJ4Y': {
    symbol: 'POYE',
    name: 'Poye',
    icon: 'https://stellar.myfilebase.com/ipfs/QmYUeh7TRHB6bbBrZ2JLSsVEgww4fAHfPzaa7gFDDXrPB4',
    decimals: 7,
  },
  // Lumenswap LSP
  'CBXE6V454EUYWVQCI4TCSOG4CSNPQ2BLYOTKAKXYFHO3KNVX4CXYCY2T': {
    symbol: 'LSP',
    name: 'Lumenswap asset',
    icon: 'https://ipfs.io/ipfs/bafkreiazthj4bttq75cxhgaekfxnv7ub6b2hq6qr5qkea4bhy4gie4qegy',
    decimals: 7,
  },
  // Lumenaire LMNR
  'CBY4MSZXK5L4HDMJHDXQLNLOA5MM5BIGCHQYMRG7ZAFY34UNU4UXPEJJ': {
    symbol: 'LMNR',
    name: 'lumenaire',
    icon: 'https://cdn.lu.meme/prod/meme/meme-aJFJRH1LcIxvTXATPI81Yair5DTVTW56.jpeg',
    decimals: 7,
  },
  // NGNC Coin
  'CBYFV4W2LTMXYZ3XWFX5BK2BY255DU2DSXNAE4FJ5A5VYUWGIBJDOIGG': {
    symbol: 'NGNC',
    name: 'NGNC Coin',
    icon: 'https://uploads-ssl.webflow.com/60a70a1080cf2974d4b1595e/61961ce43c530394bcb05349_NGRC.png',
    decimals: 7,
  },
  // Orbit USD
  'CBZPEXQLJCGUYTAQRQ4FGCXUV5O4TZER5WSOMCGNDNIIO4EJ4FU5GQNZ': {
    symbol: 'oUSD',
    name: 'Orbit USD',
    icon: 'https://testnet.orbitcdp.finance/icons/tokens/ousd.svg',
    decimals: 7,
  },
  // yXLM by Ultra Capital
  'CBZVSNVB55ANF24QVJL2K5QCLOAB6XITGTGXYEAF6NPTXYKEJUYQOHFC': {
    symbol: 'yXLM',
    name: 'yXLM by Ultra Capital',
    icon: 'https://ipfs.io/ipfs/bafkreihntcz2lpaxawmbhwidtuifladkgew6olwuly2dz5pewqillhhpay',
    decimals: 7,
  },
  // PYUSD
  'CCCRWH6Q3FNP3I2I57BDLM5AFAT7O6OF6GKQOC6SSJNDAVRZ57SPHGU2': {
    symbol: 'PYUSD',
    name: 'PYUSD',
    icon: 'https://424565.fs1.hubspotusercontent-na1.net/hubfs/424565/PYUSDLOGO.png',
    decimals: 7,
  },
  // ARS by Anclap
  'CCD6H4LBTHAPY3NGEE6TLLRUSPJGX4K5XI2J6E4MUNDB5TNXEKC23H5B': {
    symbol: 'ARS',
    name: 'ARS by Anclap',
    icon: 'https://static.anclap.com/coin/ars.png',
    decimals: 7,
  },
  // AFREUM
  'CCG27OZ5AV4WUXS6XTECWAXEY5UOMEFI2CWFA3LHZGBTLYZWTJF3MJYQ': {
    symbol: 'AFR',
    name: 'AFREUM',
    icon: 'https://ipfs.io/ipfs/bafkreibwqfiennh5grnq2jx4632227sacotxyxww7jkfo2jblelnnqite4',
    decimals: 7,
  },
  // Scopuly
  'CCJVS6IVXAAXWCMFVK6QLWHZHR4RTVRSEZRQ53GOAEDN3VY2BLPVY72J': {
    symbol: 'SCOP',
    name: 'Scopuly',
    icon: 'https://ipfs.io/ipfs/bafkreigzc5scjuh2pry7j6eqnbao7irznrf7un5jj3fwhae3qseupvpska',
    decimals: 7,
  },
  // Stronghold SHx
  'CCKCKCPHYVXQD4NECBFJTFSCU2AMSJGCNG4O6K4JVRE2BLPR7WNDBQIQ': {
    symbol: 'SHX',
    name: 'Stronghold SHx',
    icon: 'https://ipfs.io/ipfs/bafkreiafy5nqmyfy5t3ay5tgmy5wnzkqlyqkir2owjq4jqwcxklne4xqcm',
    decimals: 7,
  },
  // USDC
  'CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75': {
    symbol: 'USDC',
    name: 'USD Coin',
    icon: 'https://ipfs.io/ipfs/bafkreibpzncuhbk5ozhdw7xkcdoyf3xhwhcwcf6sj7axjzimxw6vm6pvyy',
    decimals: 7,
  },
  // BLND
  'CD25MNVTZDL4Y3XBCPCJXGXATV5WUHHOWMYFF4YBEGU5FCPGMYTVG5JY': {
    symbol: 'BLND',
    name: 'Blend',
    icon: 'https://stellar.myfilebase.com/ipfs/QmaDc2ArxQom1bFkE7JsqqZ2VdThvLV2GTRimaeJcpxU8L',
    decimals: 7,
  },
  // STROOPY
  'CDDL6HGNYGVRCQB37DDFTIFYMGTX36AQMCXI6U3NOAO2BBZDNGKMGQSY': {
    symbol: 'STROOPY',
    name: 'STROOPY',
    icon: 'https://xlmeme.com/cdn-cgi/imagedelivery/BMsStEZme-xi9eCyEb_01g/bfb4bba0-00d6-434b-ea8a-89d733dcd000/public',
    decimals: 7,
  },
  // Stellarcarbon CARBON
  'CDDS7IQJGQ2ZMO66E3MUYXZ56H2OO7RBTTAGZLZKOEA4EXCGZX65JGA7': {
    symbol: 'CARBON',
    name: 'Stellarcarbon CARBON tCO₂e',
    icon: 'https://ipfs.io/ipfs/QmezCX5JCXBWZNej8ui6TE9H764Vi13UWy1a8bE6TDP7uz',
    decimals: 3,
  },
  // USDx
  'CDIKURWHYS4FFTR5KOQK6MBFZA2K3E26WGBQI6PXBYWZ4XIOPJHDFJKP': {
    symbol: 'USDx',
    name: 'Decentralized USD Coin',
    icon: 'https://stellar.myfilebase.com/ipfs/QmQqpy1GLcGqqsg6DaHqLQntPPuFcdHZYG2pLBpHohvCUd',
    decimals: 7,
  },
  // yUSDC
  'CDOFW7HNKLUZRLFZST4EW7V3AV4JI5IHMT6BPXXSY2IEFZ4NE5TWU2P4': {
    symbol: 'yUSDC',
    name: 'yUSDC',
    icon: 'https://ipfs.io/ipfs/bafkreicvkevokwxgvqn6vjmf4hf2krk6txy3igi7ei6dwvleb5hqe4odcy',
    decimals: 7,
  },
  // EURC
  'CDTKPWPLOURQA2SGTKTUQOWRCBZEORB4BWBOMJ3D3ZTQQSGE5F6JBQLV': {
    symbol: 'EURC',
    name: 'EUR Coin',
    icon: 'https://ipfs.io/ipfs/bafkreidizqlammdzrrurfq3o5owta77fiyonn6ri72h6rltqimn2xs2by4',
    decimals: 7,
  },
  // yETH
  'CDYEOOVL6WV4JRY45CXQKOBJFFAPOM5KNQCCDNM333L6RM2L4RO3LKYG': {
    symbol: 'yETH',
    name: 'yETH By Ultra Capital',
    icon: 'https://ipfs.io/ipfs/bafkreihqsw7iai5gcfwvgnexdcwvftiv7fjjbg2skc2wcdygfngllv5ute',
    decimals: 7,
  },
};

export function getTokenIcon(contractAddress: string): string | null {
  return TOKEN_ICONS[contractAddress]?.icon ?? null;
}

export function getTokenInfo(contractAddress: string): TokenInfo | null {
  return TOKEN_ICONS[contractAddress] ?? null;
}

export function getTokenSymbol(contractAddress: string): string {
  return TOKEN_ICONS[contractAddress]?.symbol ?? 'TOKEN';
}

export function getTokenDecimals(contractAddress: string): number {
  return TOKEN_ICONS[contractAddress]?.decimals ?? 7;
}
