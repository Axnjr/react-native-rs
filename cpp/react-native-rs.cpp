#include "react-native-rs.h"

namespace reactnativers {
	const char* execute(const char *cmd) {
		return rust_execute(cmd);
	}

	void free_string(const char *ptr) {
		return rust_free_string(ptr);
	}
}
