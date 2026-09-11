// string: devin-session-token
// function: FUN_1036bd620 @ 1036bd620


void FUN_1036bd620(long *param_1)

{
  long *plVar1;
  long local_88;
  long lStack_80;
  long local_78;
  undefined8 *local_70;
  long lStack_68;
  void *local_60;
  long lStack_58;
  undefined8 *local_50;
  undefined8 *puStack_48;
  void *local_40;
  undefined8 *puStack_38;
  char *local_30;
  long lStack_28;
  
  local_50 = _malloc(0x140);
  if (local_50 != (undefined8 *)0x0) {
    *local_50 = "-----BEGIN [A-Z ]*PRIVATE KEY-----[\\s\\S]*?-----END [A-Z ]*PRIVATE KEY-----";
    local_50[1] = 0x4a;
    local_50[2] = "[redacted]";
    local_50[3] = 10;
    local_50[4] = "devin-session-token\\$[A-Za-z0-9._\\-]+";
    local_50[5] = 0x25;
    local_50[6] = "[redacted]";
    local_50[7] = 10;
    local_50[8] = "(?:gh[pousr]_|github_pat_)[A-Za-z0-9_]{16,}";
    local_50[9] = 0x2b;
    local_50[10] = "[redacted]";
    local_50[0xb] = 10;
    local_50[0xc] = "sk-(?:ant-)?[A-Za-z0-9_\\-]{16,}";
    local_50[0xd] = 0x1f;
    local_50[0xe] = "[redacted]";
    local_50[0xf] = 10;
    local_50[0x10] = "xox[baprs]-[A-Za-z0-9\\-]{8,}";
    local_50[0x11] = 0x1c;
    local_50[0x12] = "[redacted]";
    local_50[0x13] = 10;
    local_50[0x14] = "AKIA[0-9A-Z]{16}";
    local_50[0x15] = 0x10;
    local_50[0x16] = "[redacted]";
    local_50[0x17] = 10;
    local_50[0x18] = "ey[A-Za-z0-9_\\-]{8,}\\.[A-Za-z0-9_\\-]{8,}\\.[A-Za-z0-9_\\-]{8,}";
    local_50[0x19] = 0x3c;
    local_50[0x1a] = "[redacted]";
    local_50[0x1b] = 10;
    local_50[0x1c] = "(?i)bearer\\s+[A-Za-z0-9._\\-]{20,}";
    local_50[0x1d] = 0x21;
    local_50[0x1e] = "Bearer [redacted]";
    local_50[0x1f] = 0x11;
    local_50[0x20] = "([A-Za-z][A-Za-z0-9+.\\-]*://)([A-Za-z0-9._~%+\\-]*):([^\\s/\"\'`<>{}]*)@";
    local_50[0x21] = 0x44;
    local_50[0x22] = "${1}${2}:[redacted]@";
    local_50[0x23] = 0x14;
    local_50[0x24] = "data:[a-z]+/[A-Za-z0-9.+\\-]+;base64,[A-Za-z0-9+/=]+";
    local_50[0x25] = 0x33;
    local_50[0x26] = "[binary content omitted]";
    local_50[0x27] = 0x18;
    puStack_38 = local_50 + 0x28;
    local_40 = (void *)0xa;
    puStack_48 = local_50;
    FUN_1036ce1d0(&local_88,&local_50);
    FUN_1040f9858(&local_70,
                  "(?i)\\b([A-Za-z0-9_]*(?:secret|token|password|passwd|api[_-]?key|access[_-]?key|private[_-]?key|credential)[A-Za-z0-9_]*)(\\s*[:=]\\s*)\\S{6,}"
                  ,0x8a);
    if (local_70 == (undefined8 *)0x0) {
      if ((lStack_68 != -1) && (lStack_68 != 0)) {
        _free(local_60);
      }
    }
    else {
      puStack_48 = (undefined8 *)lStack_68;
      local_50 = local_70;
      puStack_38 = (undefined8 *)lStack_58;
      local_40 = local_60;
      local_30 = "$1$2[redacted]";
      lStack_28 = 0xe;
      if (local_78 == local_88) {
        FUN_105b5c404(&local_88);
      }
      plVar1 = (long *)(lStack_80 + local_78 * 0x30);
      plVar1[1] = (long)puStack_48;
      *plVar1 = (long)local_50;
      plVar1[3] = (long)puStack_38;
      plVar1[2] = (long)local_40;
      plVar1[5] = lStack_28;
      plVar1[4] = (long)local_30;
      local_78 = local_78 + 1;
    }
    param_1[1] = lStack_80;
    *param_1 = local_88;
    param_1[2] = local_78;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_105ae9b6c(8,0x140);
}

