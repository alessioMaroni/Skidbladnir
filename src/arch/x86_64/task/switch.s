.global __switch

// Intel sintax

__switch:
    // 1. Save callee-saved registers of the outgoing task onto its stack
    push rbp
    push rbx
    push r12
    push r13
    push r14
    push r15

    // 2. Store current stack pointer into outgoing task's TCB (*rdi = rsp)
    mov [rdi], rsp

    // 3. Load new stack pointer from incoming task's TCB (rsp = *rsi)
    mov rsp, [rsi]

    // 4. Restore callee-saved registers (reverse push order)
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp

    // 5. Return to new RIP
    ret