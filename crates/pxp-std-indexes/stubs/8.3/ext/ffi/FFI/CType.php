<?php

namespace FFI;

final class CType
{
    public function getName(): string
    {
    }
    public function getKind(): int
    {
    }
    public function getSize(): int
    {
    }
    public function getAlignment(): int
    {
    }
    public function getAttributes(): int
    {
    }
    public function getEnumKind(): int
    {
    }
    public function getArrayElementType(): CType
    {
    }
    public function getArrayLength(): int
    {
    }
    public function getPointerType(): CType
    {
    }
    public function getStructFieldNames(): array
    {
    }
    public function getStructFieldOffset(string $name): int
    {
    }
    public function getStructFieldType(string $name): CType
    {
    }
    public function getFuncABI(): int
    {
    }
    public function getFuncReturnType(): CType
    {
    }
    public function getFuncParameterCount(): int
    {
    }
    public function getFuncParameterType(int $index): CType
    {
    }
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_VOID
     */
    public const TYPE_VOID = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_VOID */
    public const int TYPE_VOID = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_FLOAT
     */
    public const TYPE_FLOAT = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_FLOAT */
    public const int TYPE_FLOAT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_DOUBLE
     */
    public const TYPE_DOUBLE = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_DOUBLE */
    public const int TYPE_DOUBLE = UNKNOWN;
    #ifdef HAVE_LONG_DOUBLE
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_LONGDOUBLE
     */
    public const TYPE_LONGDOUBLE = UNKNOWN;
    #ifdef HAVE_LONG_DOUBLE
    /** @cvalue ZEND_FFI_TYPE_LONGDOUBLE */
    public const int TYPE_LONGDOUBLE = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT8
     */
    public const TYPE_UINT8 = UNKNOWN;
    #endif
    /** @cvalue ZEND_FFI_TYPE_UINT8 */
    public const int TYPE_UINT8 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT8
     */
    public const TYPE_SINT8 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT8 */
    public const int TYPE_SINT8 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT16
     */
    public const TYPE_UINT16 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_UINT16 */
    public const int TYPE_UINT16 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT16
     */
    public const TYPE_SINT16 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT16 */
    public const int TYPE_SINT16 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT32
     */
    public const TYPE_UINT32 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_UINT32 */
    public const int TYPE_UINT32 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT32
     */
    public const TYPE_SINT32 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT32 */
    public const int TYPE_SINT32 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT64
     */
    public const TYPE_UINT64 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_UINT64 */
    public const int TYPE_UINT64 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT64
     */
    public const TYPE_SINT64 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT64 */
    public const int TYPE_SINT64 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_ENUM
     */
    public const TYPE_ENUM = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_ENUM */
    public const int TYPE_ENUM = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_BOOL
     */
    public const TYPE_BOOL = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_BOOL */
    public const int TYPE_BOOL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_CHAR
     */
    public const TYPE_CHAR = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_CHAR */
    public const int TYPE_CHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_POINTER
     */
    public const TYPE_POINTER = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_POINTER */
    public const int TYPE_POINTER = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_FUNC
     */
    public const TYPE_FUNC = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_FUNC */
    public const int TYPE_FUNC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_ARRAY
     */
    public const TYPE_ARRAY = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_ARRAY */
    public const int TYPE_ARRAY = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_STRUCT
     */
    public const TYPE_STRUCT = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_STRUCT */
    public const int TYPE_STRUCT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_CONST
     */
    public const ATTR_CONST = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_CONST */
    public const int ATTR_CONST = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_INCOMPLETE_TAG
     */
    public const ATTR_INCOMPLETE_TAG = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_INCOMPLETE_TAG */
    public const int ATTR_INCOMPLETE_TAG = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_VARIADIC
     */
    public const ATTR_VARIADIC = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_VARIADIC */
    public const int ATTR_VARIADIC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_INCOMPLETE_ARRAY
     */
    public const ATTR_INCOMPLETE_ARRAY = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_INCOMPLETE_ARRAY */
    public const int ATTR_INCOMPLETE_ARRAY = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_VLA
     */
    public const ATTR_VLA = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_VLA */
    public const int ATTR_VLA = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_UNION
     */
    public const ATTR_UNION = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_UNION */
    public const int ATTR_UNION = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_PACKED
     */
    public const ATTR_PACKED = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_PACKED */
    public const int ATTR_PACKED = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_MS_STRUCT
     */
    public const ATTR_MS_STRUCT = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_MS_STRUCT */
    public const int ATTR_MS_STRUCT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_GCC_STRUCT
     */
    public const ATTR_GCC_STRUCT = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_GCC_STRUCT */
    public const int ATTR_GCC_STRUCT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_DEFAULT
     */
    public const ABI_DEFAULT = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_DEFAULT */
    public const int ABI_DEFAULT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_CDECL
     */
    public const ABI_CDECL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_CDECL */
    public const int ABI_CDECL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_FASTCALL
     */
    public const ABI_FASTCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_FASTCALL */
    public const int ABI_FASTCALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_THISCALL
     */
    public const ABI_THISCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_THISCALL */
    public const int ABI_THISCALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_STDCALL
     */
    public const ABI_STDCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_STDCALL */
    public const int ABI_STDCALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_PASCAL
     */
    public const ABI_PASCAL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_PASCAL */
    public const int ABI_PASCAL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_REGISTER
     */
    public const ABI_REGISTER = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_REGISTER */
    public const int ABI_REGISTER = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_MS
     */
    public const ABI_MS = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_MS */
    public const int ABI_MS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_SYSV
     */
    public const ABI_SYSV = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_SYSV */
    public const int ABI_SYSV = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_VECTORCALL
     */
    public const ABI_VECTORCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_VECTORCALL */
    public const int ABI_VECTORCALL = UNKNOWN;
}