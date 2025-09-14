package com.reactnativers;

import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.Promise;

abstract class ReactNativeRsSpec extends ReactContextBaseJavaModule {
  ReactNativeRsSpec(ReactApplicationContext context) {
    super(context);
  }

  public abstract void execute(String cmd, Promise promise);
  public abstract void getAppHomeDir(Promise promise);
}
