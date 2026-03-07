#ifndef CODE_TRANSLATOR_H
#define CODE_TRANSLATOR_H

char *ct_translate(const char *source_code, const char *source_language, const char *target_language);
void ct_free_string(char *ptr);

#endif
