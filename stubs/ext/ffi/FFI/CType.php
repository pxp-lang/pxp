<?php 

namespace FFI;

final class CType
{
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_VOID
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_VOID = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_FLOAT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_FLOAT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_DOUBLE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_DOUBLE = UNKNOWN;
    #ifdef HAVE_LONG_DOUBLE
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_LONGDOUBLE
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_LONGDOUBLE = UNKNOWN;
    #endif
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT8
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_UINT8 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT8
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_SINT8 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT16
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_UINT16 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT16
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_SINT16 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT32
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_UINT32 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT32
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_SINT32 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_UINT64
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_UINT64 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_SINT64
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_SINT64 = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_ENUM
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_ENUM = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_BOOL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_BOOL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_CHAR
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_CHAR = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_POINTER
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_POINTER = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_FUNC
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_FUNC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_ARRAY
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_ARRAY = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_TYPE_STRUCT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const TYPE_STRUCT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_CONST
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_CONST = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_INCOMPLETE_TAG
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_INCOMPLETE_TAG = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_VARIADIC
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_VARIADIC = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_INCOMPLETE_ARRAY
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_INCOMPLETE_ARRAY = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_VLA
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_VLA = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_UNION
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_UNION = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_PACKED
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_PACKED = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_MS_STRUCT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_MS_STRUCT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ATTR_GCC_STRUCT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ATTR_GCC_STRUCT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_DEFAULT
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_DEFAULT = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_CDECL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_CDECL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_FASTCALL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_FASTCALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_THISCALL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_THISCALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_STDCALL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_STDCALL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_PASCAL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_PASCAL = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_REGISTER
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_REGISTER = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_MS
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_MS = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_SYSV
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_SYSV = UNKNOWN;
    /**
     * @var int
     * @cvalue ZEND_FFI_ABI_VECTORCALL
     */
    #[\Since('8.2')]
    #[\Until('8.3')]
    public const ABI_VECTORCALL = UNKNOWN;
    public function getName(): string
    {
    }
    #[\Since('8.1')]
    public function getKind(): int
    {
    }
    #[\Since('8.1')]
    public function getSize(): int
    {
    }
    #[\Since('8.1')]
    public function getAlignment(): int
    {
    }
    #[\Since('8.1')]
    public function getAttributes(): int
    {
    }
    #[\Since('8.1')]
    public function getEnumKind(): int
    {
    }
    #[\Since('8.1')]
    public function getArrayElementType(): CType
    {
    }
    #[\Since('8.1')]
    public function getArrayLength(): int
    {
    }
    #[\Since('8.1')]
    public function getPointerType(): CType
    {
    }
    #[\Since('8.1')]
    public function getStructFieldNames(): array
    {
    }
    #[\Since('8.1')]
    public function getStructFieldOffset(string $name): int
    {
    }
    #[\Since('8.1')]
    public function getStructFieldType(string $name): CType
    {
    }
    #[\Since('8.1')]
    public function getFuncABI(): int
    {
    }
    #[\Since('8.1')]
    public function getFuncReturnType(): CType
    {
    }
    #[\Since('8.1')]
    public function getFuncParameterCount(): int
    {
    }
    #[\Since('8.1')]
    public function getFuncParameterType(int $index): CType
    {
    }
    /** @cvalue ZEND_FFI_TYPE_VOID */
    #[\Since('8.3')]
    public const int TYPE_VOID = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_FLOAT */
    #[\Since('8.3')]
    public const int TYPE_FLOAT = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_DOUBLE */
    #[\Since('8.3')]
    public const int TYPE_DOUBLE = UNKNOWN;
    #ifdef HAVE_LONG_DOUBLE
    /** @cvalue ZEND_FFI_TYPE_LONGDOUBLE */
    #[\Since('8.3')]
    public const int TYPE_LONGDOUBLE = UNKNOWN;
    #endif
    /** @cvalue ZEND_FFI_TYPE_UINT8 */
    #[\Since('8.3')]
    public const int TYPE_UINT8 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT8 */
    #[\Since('8.3')]
    public const int TYPE_SINT8 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_UINT16 */
    #[\Since('8.3')]
    public const int TYPE_UINT16 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT16 */
    #[\Since('8.3')]
    public const int TYPE_SINT16 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_UINT32 */
    #[\Since('8.3')]
    public const int TYPE_UINT32 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT32 */
    #[\Since('8.3')]
    public const int TYPE_SINT32 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_UINT64 */
    #[\Since('8.3')]
    public const int TYPE_UINT64 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_SINT64 */
    #[\Since('8.3')]
    public const int TYPE_SINT64 = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_ENUM */
    #[\Since('8.3')]
    public const int TYPE_ENUM = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_BOOL */
    #[\Since('8.3')]
    public const int TYPE_BOOL = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_CHAR */
    #[\Since('8.3')]
    public const int TYPE_CHAR = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_POINTER */
    #[\Since('8.3')]
    public const int TYPE_POINTER = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_FUNC */
    #[\Since('8.3')]
    public const int TYPE_FUNC = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_ARRAY */
    #[\Since('8.3')]
    public const int TYPE_ARRAY = UNKNOWN;
    /** @cvalue ZEND_FFI_TYPE_STRUCT */
    #[\Since('8.3')]
    public const int TYPE_STRUCT = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_CONST */
    #[\Since('8.3')]
    public const int ATTR_CONST = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_INCOMPLETE_TAG */
    #[\Since('8.3')]
    public const int ATTR_INCOMPLETE_TAG = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_VARIADIC */
    #[\Since('8.3')]
    public const int ATTR_VARIADIC = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_INCOMPLETE_ARRAY */
    #[\Since('8.3')]
    public const int ATTR_INCOMPLETE_ARRAY = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_VLA */
    #[\Since('8.3')]
    public const int ATTR_VLA = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_UNION */
    #[\Since('8.3')]
    public const int ATTR_UNION = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_PACKED */
    #[\Since('8.3')]
    public const int ATTR_PACKED = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_MS_STRUCT */
    #[\Since('8.3')]
    public const int ATTR_MS_STRUCT = UNKNOWN;
    /** @cvalue ZEND_FFI_ATTR_GCC_STRUCT */
    #[\Since('8.3')]
    public const int ATTR_GCC_STRUCT = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_DEFAULT */
    #[\Since('8.3')]
    public const int ABI_DEFAULT = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_CDECL */
    #[\Since('8.3')]
    public const int ABI_CDECL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_FASTCALL */
    #[\Since('8.3')]
    public const int ABI_FASTCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_THISCALL */
    #[\Since('8.3')]
    public const int ABI_THISCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_STDCALL */
    #[\Since('8.3')]
    public const int ABI_STDCALL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_PASCAL */
    #[\Since('8.3')]
    public const int ABI_PASCAL = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_REGISTER */
    #[\Since('8.3')]
    public const int ABI_REGISTER = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_MS */
    #[\Since('8.3')]
    public const int ABI_MS = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_SYSV */
    #[\Since('8.3')]
    public const int ABI_SYSV = UNKNOWN;
    /** @cvalue ZEND_FFI_ABI_VECTORCALL */
    #[\Since('8.3')]
    public const int ABI_VECTORCALL = UNKNOWN;
}