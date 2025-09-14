#import "ReactNativeRs.h"

@implementation ReactNativeRs
RCT_EXPORT_MODULE()

RCT_EXPORT_METHOD(getAppHomeDir
  :(RCTPromiseResolveBlock)resolve
  rejecter:(RCTPromiseRejectBlock)reject)
{
    @try {
        NSArray *paths = NSSearchPathForDirectoriesInDomains(NSDocumentDirectory, NSUserDomainMask, YES);
        NSString *documentsDirectory = [paths firstObject];
        resolve(documentsDirectory);
    }
    @catch (NSException *exception) {
        reject(@"get_app_home_dir_error", @"Failed to get the app home directory", nil);
    }
}

RCT_EXPORT_METHOD(execute:(NSString*)cmd
  resolve:(RCTPromiseResolveBlock)resolve 
  reject:(RCTPromiseRejectBlock)reject)
{
    dispatch_async(dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
        const char* raw = reactnativers::execute([cmd UTF8String]);
        NSString *result = [NSString stringWithUTF8String:raw];
        reactnativers::free_string(raw);
        if (result == nil) {
            dispatch_async(dispatch_get_main_queue(), ^{
                reject(@"EXEC_ERROR", @"Execution returned nil", nil);
            });
            return;
        }
        dispatch_async(dispatch_get_main_queue(), ^{
            resolve(result);
        });
    });
}

// Don't compile this code when we build for the old architecture.
#ifdef RCT_NEW_ARCH_ENABLED
- (std::shared_ptr<facebook::react::TurboModule>)getTurboModule:
    (const facebook::react::ObjCTurboModule::InitParams &)params
{
    return std::make_shared<facebook::react::NativeReactNativeRsSpecJSI>(params);
}
#endif

@end
