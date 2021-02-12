#pragma once 
#include "php.h"

// workaround for bindgen not exporting ZEND_MODULE_BUILD_ID correctly
static const char *ZEND_MODULE_BUILD_ID_ = ZEND_MODULE_BUILD_ID;
