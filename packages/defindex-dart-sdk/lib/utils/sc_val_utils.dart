import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';


Map<String, dynamic> parseMap(XdrSCVal scval) {
  if (scval.discriminant != XdrSCValType.SCV_MAP) {
    throw Exception('Expected Map type, got ${scval.discriminant}');
  }
  Map<String, dynamic> result = {};
  for (var entry in scval.map!) {
    String key = parseScVal(entry.key);
    dynamic value = parseScVal(entry.val);

    result[key] = value;
  }
  return result;
}


BigInt int128PartsToBigInt(XdrInt128Parts value) {
  BigInt hi = BigInt.from(value.hi.int64);
  BigInt lo = BigInt.from(value.lo.uint64);
  return (hi << 64) | lo;
}
/// Parsea un XdrSCVal a su tipo Dart correspondiente
dynamic parseScVal(XdrSCVal val) {
  switch (val.discriminant) {
    case XdrSCValType.SCV_BOOL:
      return val.b;
    case XdrSCValType.SCV_U32:
      return BigInt.from(val.u32!.uint32);
    case XdrSCValType.SCV_I32:
      return BigInt.from(val.i32!.int32);
    case XdrSCValType.SCV_U64:
      return BigInt.from(val.u64!.uint64);
    case XdrSCValType.SCV_I64:
      return BigInt.from(val.i64!.int64);
    case XdrSCValType.SCV_U128:
      return BigInt.from(val.u128!.lo.uint64);
    case XdrSCValType.SCV_I128:
      return int128PartsToBigInt(val.i128!);
    case XdrSCValType.SCV_STRING:
      return val.str;
    case XdrSCValType.SCV_BYTES:
      return val.bytes;
    case XdrSCValType.SCV_ADDRESS:
      try {
        if (val.address?.contractId != null) {
          final contractIdHex = Util.bytesToHex(val.address!.contractId!.hash);
          final strKeyContractId = StrKey.encodeContractIdHex(contractIdHex);
          return strKeyContractId;
        } else {
          throw Exception('Invalid address format');
        }
      } catch (e) {
        throw Exception('Invalid address format');
      }
    case XdrSCValType.SCV_SYMBOL:
      return val.sym;
    case XdrSCValType.SCV_VEC:
      return val.vec?.map((e) => parseScVal(e)).toList();
    case XdrSCValType.SCV_MAP:
      return parseMap(val);
    default:
      throw Exception('Unsupported type: ${val.discriminant}');
  }
}
