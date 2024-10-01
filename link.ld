MEMORY {
	FLASH : ORIGIN = 0x00000000 + 16K, LENGTH = 512K - 16K
	RAM : ORIGIN = 0x20000000, LENGTH = 192K
}

SECTIONS {
	.vector_table ORIGIN(FLASH) : {
		/* initial stack pointer */
		LONG(ORIGIN(RAM) + LENGTH(RAM));

		/* exceptions */
		KEEP(*(.vector_table.exceptions));
	} > FLASH

	.text : {
		*(.text .text.*);
	} > FLASH
}
