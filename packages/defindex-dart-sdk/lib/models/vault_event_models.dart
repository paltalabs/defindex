import 'dart:math' as math;

import 'package:defindex_sdk/defindex_sdk.dart';

class VaultEventPair {
  final VaultEvent recent;
  final VaultEvent past;
  
  VaultEventPair(this.recent, this.past);
  
  double calculateAPY() {
    final recentPPS = recent.calculatePricePerShare();
    final pastPPS = past.calculatePricePerShare();

    print('Recent PPS: $recentPPS, Past PPS: $pastPPS');

    if (recentPPS <= 0 || pastPPS <= 0) return 0.0;

    final secondsDiff = recent.date.difference(past.date).inSeconds;
    if (secondsDiff <= 0) return 0.0;
    
    final dailyRate = (recentPPS / pastPPS - 1) * 86400 / secondsDiff;
    
    final apy = math.pow(1 + dailyRate, 365.2425) - 1;
    
    return (apy as double).clamp(0.0, double.infinity);
  }
}
