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

    .rodata : {
        *(.rodata .rodata.*);
    } > FLASH

    .bss (NOLOAD) : {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > RAM

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata)) {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > RAM

    _sidata = LOADADDR(.data);

    .ps_stack ALIGN(0x08) : {
        KEEP(*(.ps_stack .ps_stack.*));
    } > RAM

    .ps_stack2 ALIGN(0x08) : {
        KEEP(*(.ps_stack2 .ps_stack2.*));
    } > RAM
}
