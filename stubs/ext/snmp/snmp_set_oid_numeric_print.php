<?php 

/** @alias snmp_set_oid_output_format */
#[\Until('8.3')]
function snmp_set_oid_numeric_print(int $format): bool
{
}
/** @alias snmp_set_oid_output_format */
#[\Since('8.3')]
function snmp_set_oid_numeric_print(int $format): true
{
}