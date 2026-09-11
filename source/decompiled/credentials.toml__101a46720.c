// string: credentials.toml
// function: FUN_101a46720 @ 101a46720


void FUN_101a46720(long *param_1)

{
  long lVar1;
  void *pvVar2;
  long local_60;
  void *local_58;
  undefined8 uStack_50;
  long local_48;
  void *pvStack_40;
  long local_38;
  
  FUN_1052c0d9c(&local_60);
  if (local_60 == -1) {
    pvStack_40 = (void *)FUN_105b42a30("Failed to determine data directory",0x22);
  }
  else {
    FUN_10464cd20(&local_48,local_58,uStack_50,"devin",5);
    if (local_60 != 0) {
      _free(local_58);
    }
    pvVar2 = pvStack_40;
    lVar1 = local_48;
    if (local_48 != -1) {
      FUN_10464cd20(&local_48,pvStack_40,local_38,"credentials.toml",0x10);
      param_1[1] = (long)pvStack_40;
      *param_1 = local_48;
      param_1[2] = local_38;
      if (lVar1 == 0) {
        return;
      }
      _free(pvVar2);
      return;
    }
  }
  *param_1 = -1;
  param_1[1] = (long)pvStack_40;
  return;
}

