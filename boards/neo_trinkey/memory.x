MEMORY
{
  /* Leave 8k for the default bootloader on the Neo Trinkey */
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

