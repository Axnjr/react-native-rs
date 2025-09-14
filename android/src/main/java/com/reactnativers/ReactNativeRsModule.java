package com.reactnativers;

import android.util.Log;
import androidx.annotation.NonNull;
import android.content.Context;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactMethod;
import java.io.File;

public class ReactNativeRsModule extends ReactNativeRsSpec {
  public static final String NAME = "ReactNativeRs";

  ReactNativeRsModule(ReactApplicationContext context) {
    super(context);
    Log.d(NAME, "ReactNativeRs module initialised");
  }

  @Override
  @NonNull
  public String getName() {
    Log.i(NAME, "ReactNativeRs getName called, returning " + NAME);
    return NAME;
  }
  
  static {
    try {
      Log.i(NAME, "About to load react-native-rs");
      System.loadLibrary("react-native-rs");
      Log.i(NAME, "react-native-rs loaded successfully");
    } catch (Throwable err) {
      Log.e(NAME, "Failed to load react-native-rs", err);
    }
  }

  public static native String nativeExecute(String cmd);

  @ReactMethod
  public void getAppHomeDir(Promise promise) {
    Log.i(NAME, "getAppHomeDir called");
    try {
      // https://developer.android.com/reference/android/content/Context#getFilesDir()
      File homeDir = getReactApplicationContext().getFilesDir();
      if (!homeDir.exists()) {
        Log.e(NAME, "Failed to get home directory");
        promise.reject("NO_DIR", "home dir is unavailable");
      }

      promise.resolve(homeDir.getAbsolutePath());
    } catch (Throwable err) {
      Log.e(NAME, "Error while initialising Android context: ", err);
      promise.reject("NO_DIR", "home dir is unavailable due to error");
    }
  }

  @ReactMethod
  public void execute(final String cmd, final Promise promise) {
    Log.i(NAME, "execute called");
    new Thread(() -> {
      try {
        String cmdRes = nativeExecute(cmd);
        promise.resolve(cmdRes);
      } catch (Throwable err) {
        Log.e(NAME, "Error while executing command: ", err);
        promise.reject("EXEC_ERROR", err);
      }
    }).start();
  }
}
