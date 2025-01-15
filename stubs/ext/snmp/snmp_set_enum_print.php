<?php 

#[\Until('8.3')]
function snmp_set_enum_print(bool $enable): bool
{
}
#[\Since('8.3')]
function snmp_set_enum_print(bool $enable): true
{
}