import { existsSync, readFileSync, writeFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export interface Asset {
  name: string;
  contract: string;
  code: string;
  issuer?: string;
  icon?: string;
  decimals: number;
}

interface NetworkTokens {
  network: string;
  assets: Asset[];
}

export class TokensBook {
  private networks: NetworkTokens[];
  private fileName: string;

  constructor(networks: NetworkTokens[], fileName: string) {
    this.networks = networks;
    this.fileName = fileName;
  }

  static loadFromFile(
    tokensBookPath: string = "../../.soroban",
    fileName: string = "tokens.json"
  ) {
    const filePath = path.join(__dirname, tokensBookPath, fileName);
    let networks: NetworkTokens[];

    if (existsSync(filePath)) {
      const fileContent = readFileSync(filePath, { encoding: "utf-8" });
      networks = JSON.parse(fileContent);
    } else {
      // If the file doesn't exist, create a new empty array for networks
      networks = [
        {
          network: "mainnet",
          assets: [],
        },
        {
          network: "testnet",
          assets: [],
        },
        {
          network: "standalone",
          assets: [],
        },
      ];
    }

    return new TokensBook(networks, fileName);
  }

  writeToFile() {
    const filePath = path.join(__dirname, "../../.soroban/", this.fileName);
    const fileContent = JSON.stringify(this.networks, null, 2);
    writeFileSync(filePath, fileContent);
  }

  addToken(networkName: string, token: Asset) {
    const network = this.networks.find((n) => n.network === networkName);
    if (network) {
      const tokenExists = network.assets.some(
        (t) => t.contract === token.contract
      );

      if (!tokenExists) {
        network.assets.push(token);
      }
    } else {
      this.networks.push({
        network: networkName,
        assets: [token],
      });
    }
  }

  prependToken(networkName: string, token: Asset) {
    const network = this.networks.find((n) => n.network === networkName);
    if (network) {
      const tokenExists = network.assets.some(
        (t) => t.contract === token.contract
      );

      if (!tokenExists) {
        network.assets.unshift(token);
      }
    } else {
      this.networks.push({
        network: networkName,
        assets: [token],
      });
    }
  }

  getTokensByNetwork(networkName: string): Asset[] | undefined {
    const network = this.networks.find((n) => n.network === networkName);
    return network?.assets;
  }

  resetNetworkTokens(networkName: string) {
    const networkIndex = this.networks.findIndex(
      (n) => n.network === networkName
    );
    if (networkIndex !== -1) {
      this.networks[networkIndex].assets = [];
    } else {
      this.networks.push({
        network: networkName,
        assets: [],
      });
    }
  }
}
