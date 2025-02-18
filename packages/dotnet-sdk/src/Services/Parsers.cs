using DeFindex.Sdk.Interfaces;
using Newtonsoft.Json;
using StellarDotnetSdk.Responses.SorobanRpc;
using StellarDotnetSdk.Soroban;

public class DefindexResponseParser
{
  public static List<ManagedFundsResult> ParseManagedFundsResult(SimulateTransactionResponse response)
  {
    var managedFundsResults = new List<ManagedFundsResult>();

    if (response.Results == null || response.Results.Length == 0)
    {
        Console.WriteLine("No results found.");
        return managedFundsResults;
    }
    
    var xdrString = response.Results[0].Xdr;

    if (xdrString == null){
        Console.WriteLine("XDR string for total managed funds is null.");
        return managedFundsResults;
    }
    
    var result_xdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString));
    var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(result_xdr);
    foreach (var i in xdr.Vec.InnerValue) 
    {
      string Asset = "";
      ulong IdleAmount = 0;
      ulong InvestedAmount = 0;
      ulong TotalAmount = 0;
      List<StrategyAllocation> StrategyAllocations = new List<StrategyAllocation>();
      foreach (var j in xdr.Vec.InnerValue[0].Map.InnerValue)
      { 
          switch (j.Key.Sym.InnerValue)
          {
              case "asset":
                  var contractId = (SCContractId)SCContractId.FromSCValXdr(j.Val);
                  Asset = contractId.InnerValue;
                  break;
              case "idle_amount":
                  var idleAmount = (SCInt128)SCInt128.FromSCValXdr(j.Val);
                  IdleAmount = idleAmount.Lo;
                  break;
              case "invested_amount":
                  var investedAmount = (SCInt128)SCInt128.FromSCValXdr(j.Val);
                  InvestedAmount = investedAmount.Lo;
                  break;
              case "total_amount":
                  var totalAmount = (SCInt128)SCInt128.FromSCValXdr(j.Val);
                  TotalAmount = totalAmount.Lo;
                  break;
              case "strategy_allocations":
                  var strategyAllocations = (SCVec)SCVec.FromSCValXdr(j.Val);
                  foreach (var strategyAllocation in strategyAllocations.InnerValue)
                  {
                      var strategyAllocationsMap = (SCMap)SCMap.FromSCValXdr(strategyAllocation.ToXdr());
                      ulong Amount = 0;
                      bool Paused = false;
                      string StrategyAddress = "";
                      foreach (var entry in strategyAllocationsMap.Entries)
                      {
                          var key = (SCSymbol)SCSymbol.FromSCValXdr(entry.Key.ToXdr());
                          switch (key.InnerValue)
                          {
                              case "amount":
                                  var amount = (SCInt128)SCInt128.FromSCValXdr(entry.Value.ToXdr());
                                  Amount = amount.Lo;
                                  break;
                              case "paused":
                                  var paused = (SCBool)SCBool.FromSCValXdr(entry.Value.ToXdr());
                                  Paused = paused.InnerValue;
                                  break;
                              case "strategy_address":
                                  var strategyAddress = (SCContractId)SCContractId.FromSCValXdr(entry.Value.ToXdr());
                                  StrategyAddress = strategyAddress.InnerValue;
                                  break;
                          }
                      }
                      StrategyAllocations.Add(new StrategyAllocation(Amount, Paused, StrategyAddress));
                  }
                  break;
          }
      }
      var result = new ManagedFundsResult(Asset, IdleAmount, InvestedAmount, TotalAmount, StrategyAllocations);
      managedFundsResults.Add(result);
    }
    return managedFundsResults;
  }
  public static List<TransactionResult> ParseSubmittedTransaction(SCVal result, string txHash) 
  {
    List<ulong> Amounts = new List<ulong>();
    ulong SharesChanged = 0;
    var res = result.ToXdr().Vec.InnerValue;
    if (res.Length == 3)
    {
        var depositedAmounts = res[0];
        SharesChanged = res[1].I128.Lo.InnerValue;
        foreach (var depositedAmount in depositedAmounts.Vec.InnerValue)
        {
            var amount = (SCInt128)SCInt128.FromSCValXdr(depositedAmount);
            Amounts.Add(amount.Lo);
        }
    } 
    else if (res.Length == 1)
    {
        foreach (var i in res)
        {
            var amount = (SCInt128)SCInt128.FromSCValXdr(i);
            SharesChanged += amount.Lo;
            Amounts.Add(amount.Lo);
        }
    }
    else
    {
        Console.WriteLine("Unexpected number of results received.");
        return new List<TransactionResult>();
    }
    var response = new TransactionResult(true, txHash, Amounts, SharesChanged);
    Console.WriteLine($"Transaction result: {JsonConvert.SerializeObject(response, Formatting.Indented)}");    
    return new List<TransactionResult> { response };
  }
}