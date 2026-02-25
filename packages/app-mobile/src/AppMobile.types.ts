export type AppMobileModuleEvents = {
  onTick: (params: OnTickEventPayload) => void;
};

export type OnTickEventPayload = {
  count: number;
};
