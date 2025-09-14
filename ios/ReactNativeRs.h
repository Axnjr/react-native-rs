#ifdef __cplusplus
#import "react-native-rs.h"
#endif

#ifdef RCT_NEW_ARCH_ENABLED
#import "ReactNativeRsSpec.h"

@interface ReactNativeRs : NSObject <NativeReactNativeRsSpec>
#else
#import <React/RCTBridgeModule.h>

@interface ReactNativeRs : NSObject <RCTBridgeModule>
#endif

@end
