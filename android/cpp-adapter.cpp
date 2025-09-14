#include <jni.h>
#include "react-native-rs.h"

extern "C"
JNIEXPORT jstring JNICALL
Java_com_reactnativers_ReactNativeRsModule_nativeExecute(JNIEnv *env, jclass type, jstring cmd) {
    const char *native_cmd = env->GetStringUTFChars(cmd, JNI_FALSE);
    if (native_cmd == nullptr) {
        return nullptr;
    }
    const char *res = reactnativers::execute(native_cmd);
    env->ReleaseStringUTFChars(cmd, native_cmd);

    jstring result = env->NewStringUTF(res);
    reactnativers::free_string(res);

    return result;
}
