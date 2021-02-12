/* Generated by the Systemtap dtrace wrapper */


#define _SDT_HAS_SEMAPHORES 1


#define STAP_HAS_SEMAPHORES 1 /* deprecated */


#include <sys/sdt.h>

/* DTRACE_EXCEPTION_CAUGHT ( char *classname ) */
#if defined STAP_SDT_V1
#define DTRACE_EXCEPTION_CAUGHT_ENABLED() __builtin_expect (exception__caught_semaphore, 0)
#define php_exception__caught_semaphore exception__caught_semaphore
#else
#define DTRACE_EXCEPTION_CAUGHT_ENABLED() __builtin_expect (php_exception__caught_semaphore, 0)
#endif
__extension__ extern unsigned short php_exception__caught_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_EXCEPTION_CAUGHT(arg1) \
DTRACE_PROBE1 (php, exception__caught, arg1)

/* DTRACE_EXCEPTION_THROWN ( char* classname ) */
#if defined STAP_SDT_V1
#define DTRACE_EXCEPTION_THROWN_ENABLED() __builtin_expect (exception__thrown_semaphore, 0)
#define php_exception__thrown_semaphore exception__thrown_semaphore
#else
#define DTRACE_EXCEPTION_THROWN_ENABLED() __builtin_expect (php_exception__thrown_semaphore, 0)
#endif
__extension__ extern unsigned short php_exception__thrown_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_EXCEPTION_THROWN(arg1) \
DTRACE_PROBE1 (php, exception__thrown, arg1)

/* DTRACE_REQUEST_STARTUP ( char* request_file, char* request_uri, char* request_method ) */
#if defined STAP_SDT_V1
#define DTRACE_REQUEST_STARTUP_ENABLED() __builtin_expect (request__startup_semaphore, 0)
#define php_request__startup_semaphore request__startup_semaphore
#else
#define DTRACE_REQUEST_STARTUP_ENABLED() __builtin_expect (php_request__startup_semaphore, 0)
#endif
__extension__ extern unsigned short php_request__startup_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_REQUEST_STARTUP(arg1, arg2, arg3) \
DTRACE_PROBE3 (php, request__startup, arg1, arg2, arg3)

/* DTRACE_REQUEST_SHUTDOWN ( char* request_file, char* request_uri, char* request_method ) */
#if defined STAP_SDT_V1
#define DTRACE_REQUEST_SHUTDOWN_ENABLED() __builtin_expect (request__shutdown_semaphore, 0)
#define php_request__shutdown_semaphore request__shutdown_semaphore
#else
#define DTRACE_REQUEST_SHUTDOWN_ENABLED() __builtin_expect (php_request__shutdown_semaphore, 0)
#endif
__extension__ extern unsigned short php_request__shutdown_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_REQUEST_SHUTDOWN(arg1, arg2, arg3) \
DTRACE_PROBE3 (php, request__shutdown, arg1, arg2, arg3)

/* DTRACE_COMPILE_FILE_ENTRY ( char * compile_file, char *compile_file_translated ) */
#if defined STAP_SDT_V1
#define DTRACE_COMPILE_FILE_ENTRY_ENABLED() __builtin_expect (compile__file__entry_semaphore, 0)
#define php_compile__file__entry_semaphore compile__file__entry_semaphore
#else
#define DTRACE_COMPILE_FILE_ENTRY_ENABLED() __builtin_expect (php_compile__file__entry_semaphore, 0)
#endif
__extension__ extern unsigned short php_compile__file__entry_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_COMPILE_FILE_ENTRY(arg1, arg2) \
DTRACE_PROBE2 (php, compile__file__entry, arg1, arg2)

/* DTRACE_COMPILE_FILE_RETURN ( char *compile_file, char *compile_file_translated ) */
#if defined STAP_SDT_V1
#define DTRACE_COMPILE_FILE_RETURN_ENABLED() __builtin_expect (compile__file__return_semaphore, 0)
#define php_compile__file__return_semaphore compile__file__return_semaphore
#else
#define DTRACE_COMPILE_FILE_RETURN_ENABLED() __builtin_expect (php_compile__file__return_semaphore, 0)
#endif
__extension__ extern unsigned short php_compile__file__return_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_COMPILE_FILE_RETURN(arg1, arg2) \
DTRACE_PROBE2 (php, compile__file__return, arg1, arg2)

/* DTRACE_ERROR ( char *errormsg, char *request_file, int lineno ) */
#if defined STAP_SDT_V1
#define DTRACE_ERROR_ENABLED() __builtin_expect (error_semaphore, 0)
#define php_error_semaphore error_semaphore
#else
#define DTRACE_ERROR_ENABLED() __builtin_expect (php_error_semaphore, 0)
#endif
__extension__ extern unsigned short php_error_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_ERROR(arg1, arg2, arg3) \
DTRACE_PROBE3 (php, error, arg1, arg2, arg3)

/* DTRACE_EXECUTE_ENTRY ( char* request_file, int lineno ) */
#if defined STAP_SDT_V1
#define DTRACE_EXECUTE_ENTRY_ENABLED() __builtin_expect (execute__entry_semaphore, 0)
#define php_execute__entry_semaphore execute__entry_semaphore
#else
#define DTRACE_EXECUTE_ENTRY_ENABLED() __builtin_expect (php_execute__entry_semaphore, 0)
#endif
__extension__ extern unsigned short php_execute__entry_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_EXECUTE_ENTRY(arg1, arg2) \
DTRACE_PROBE2 (php, execute__entry, arg1, arg2)

/* DTRACE_EXECUTE_RETURN ( char* request_file, int lineno ) */
#if defined STAP_SDT_V1
#define DTRACE_EXECUTE_RETURN_ENABLED() __builtin_expect (execute__return_semaphore, 0)
#define php_execute__return_semaphore execute__return_semaphore
#else
#define DTRACE_EXECUTE_RETURN_ENABLED() __builtin_expect (php_execute__return_semaphore, 0)
#endif
__extension__ extern unsigned short php_execute__return_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_EXECUTE_RETURN(arg1, arg2) \
DTRACE_PROBE2 (php, execute__return, arg1, arg2)

/* DTRACE_FUNCTION_ENTRY ( char* function_name, char* request_file, int lineno, char* classname, char* scope ) */
#if defined STAP_SDT_V1
#define DTRACE_FUNCTION_ENTRY_ENABLED() __builtin_expect (function__entry_semaphore, 0)
#define php_function__entry_semaphore function__entry_semaphore
#else
#define DTRACE_FUNCTION_ENTRY_ENABLED() __builtin_expect (php_function__entry_semaphore, 0)
#endif
__extension__ extern unsigned short php_function__entry_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_FUNCTION_ENTRY(arg1, arg2, arg3, arg4, arg5) \
DTRACE_PROBE5 (php, function__entry, arg1, arg2, arg3, arg4, arg5)

/* DTRACE_FUNCTION_RETURN ( char* function_name, char* request_file, int lineno, char* classname, char* scope ) */
#if defined STAP_SDT_V1
#define DTRACE_FUNCTION_RETURN_ENABLED() __builtin_expect (function__return_semaphore, 0)
#define php_function__return_semaphore function__return_semaphore
#else
#define DTRACE_FUNCTION_RETURN_ENABLED() __builtin_expect (php_function__return_semaphore, 0)
#endif
__extension__ extern unsigned short php_function__return_semaphore __attribute__ ((unused)) __attribute__ ((section (".probes")));
#define DTRACE_FUNCTION_RETURN(arg1, arg2, arg3, arg4, arg5) \
DTRACE_PROBE5 (php, function__return, arg1, arg2, arg3, arg4, arg5)

