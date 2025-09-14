#ifndef REACTNATIVERS_H
#define REACTNATIVERS_H

extern "C" {
  const char *rust_execute(const char*);
  void rust_free_string(const char*);
}

namespace reactnativers {
  const char* execute(const char * cmd);
  void free_string(const char *ptr);
}

#endif /* REACTNATIVERS_H */
