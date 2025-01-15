<?php 

#endif
// TODO: $ignore_transparent is ignored???
#[\Until('8.3')]
function imagerotate(\GdImage $image, float $angle, int $background_color, bool $ignore_transparent = false): \GdImage|false
{
}
#endif
/** @refcount 1 */
#[\Since('8.3')]
function imagerotate(\GdImage $image, float $angle, int $background_color): \GdImage|false
{
}