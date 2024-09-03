interface DeFindexOptions {
  network?: string;
}

export class DeFindexSDK {
  private _network: string;

  constructor(options: DeFindexOptions) {
    this._network = options.network || "TESTNET";
  }

  public async createTransaction(params: any) {
    console.log(this._network);
    console.log(params);
  }
}
