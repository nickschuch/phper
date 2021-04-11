#ifndef PHPER_PHP_WRAPPER_H
#define PHPER_PHP_WRAPPER_H

#include <php.h>
#include <php_ini.h>
#include <ext/standard/info.h>
#include <zend_exceptions.h>

typedef ZEND_INI_MH(phper_zend_ini_mh);

zend_string *zend_string_init_(const char *str, size_t len, int persistent);
zend_string *zend_new_interned_string_(zend_string *str);
zend_class_entry phper_init_class_entry_ex(const char *class_name, size_t class_name_len, const zend_function_entry *functions);
void phper_zval_string(zval *return_value, const char *s);
zend_uchar phper_zval_get_type(const zval* pz);
void phper_zval_stringl(zval *return_value, const char *s, size_t len);
char *phper_z_strval_p(const zval *v);
zval *phper_get_this(zend_execute_data *execute_data);
void phper_zval_zval(zval *return_value, zval *zv, int copy, int dtor);
zend_string *phper_zval_get_string(zval *op);
void phper_zend_string_release(zend_string *s);
zend_long phper_zval_get_long(zval *op);

#endif //PHPER_PHP_WRAPPER_H
