using System.Numerics;
using StellarDotnetSdk.Soroban;

namespace Paltalabs.Defindex.Services
{
  public class SoroswapRouter
  {
    public DefindexHelpers Helpers = new DefindexHelpers();

    public string Address { get; set; } = "CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH";
    private async Task<string> RouterPairFor(string tokenA, string tokenB, SorobanServer server)
    {
      var tokenACID = new ScContractId(tokenA);
      var tokenBCID = new ScContractId(tokenB);
      var PairAddress = await Helpers.CallContractMethod(
        this.Address,
        "router_pair_for",
        [tokenACID, tokenBCID],
        server
      );
      if (PairAddress!.Results![0].Xdr == null)
      {
        throw new Exception("Pair address is null");
      }
      var rawAddress = PairAddress.Results[0].Xdr!;
      var xdr = ScAddress.FromXdrBase64(rawAddress);
      var pairAddress = (ScContractId)ScContractId.FromXdrBase64(xdr.ToXdrBase64());
      return pairAddress.InnerValue;
    }

    private async Task<List<string>> GetOrderedTokens(string tokenA, string tokenB, SorobanServer server)
    {
      var OrderedTokens = new List<string>();
      var Router = new SoroswapRouter();
      var PairAddress = await Router.RouterPairFor(tokenA, tokenB, server);
      var Token0 = await Helpers.CallContractMethod(
        PairAddress,
        "token_0",
        [],
        server
      );
      if (Token0!.Results![0].Xdr == null)
      {
        throw new Exception("Token0 is null");
      }
      var RawAddress = Token0.Results[0].Xdr!;
      var Xdr = ScAddress.FromXdrBase64(RawAddress);
      var Token0Address = (ScContractId)ScContractId.FromXdrBase64(Xdr.ToXdrBase64());
      var Token1 = await Helpers.CallContractMethod(
        PairAddress,
        "token_1",
        [],
        server
      );
      if (Token1!.Results![0].Xdr == null)
      {
        throw new Exception("Token1 is null");
      }
      var RawAddress1 = Token1.Results[0].Xdr!;
      var Xdr1 = ScAddress.FromXdrBase64(RawAddress1);
      var Token1Address = (ScContractId)ScContractId.FromXdrBase64(Xdr1.ToXdrBase64());

      OrderedTokens.Add(Token0Address.InnerValue);
      OrderedTokens.Add(Token1Address.InnerValue);
      return OrderedTokens;

    }

    private Dictionary<string, BigInteger> CreateReservesDict(List<string> OrderedTokens, List<BigInteger> Reserves)
    {
      var ReservesDict = new Dictionary<string, BigInteger>();
      for (int i = 0; i < OrderedTokens.Count; i++)
      {
        var token = OrderedTokens[i];
        var reserve = Reserves[i];
        ReservesDict.Add(token, reserve);
      }
      return ReservesDict;
    }

    public async Task<Dictionary<string, BigInteger>> GetPairReserves(string tokenA, string tokenB, SorobanServer server)
    {
      var Router = new SoroswapRouter();
      var PairAddress = await Router.RouterPairFor(tokenA, tokenB, server);
      Console.WriteLine($"PairAddress: {PairAddress}");
      var Reserves = await Helpers.CallContractMethod(
        PairAddress,
        "get_reserves",
        [],
        server
      );
      if (Reserves!.Results![0].Xdr == null)
      {
        throw new Exception("Reserves are null");
      }
      var RawReseves = Reserves.Results[0].Xdr!;
      var vec = (StellarDotnetSdk.Soroban.SCVec)StellarDotnetSdk.Soroban.SCVec.FromXdrBase64(RawReseves);
      var allReserves = new List<BigInteger>();
      foreach (var item in vec.InnerValue)
      {
        var reserveValue = DefindexResponseParser.ToBigInteger((SCInt128)item);
        allReserves.Add(reserveValue);
      }
      var OrderedTokens = await GetOrderedTokens(tokenA, tokenB, server);
      var ReservesDict = CreateReservesDict(OrderedTokens, allReserves);
      return ReservesDict;
    }


  }  
}