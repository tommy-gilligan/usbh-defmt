#![no_std]

use trace::trace;
use defmt::println;

use usb_device::UsbDirection;
use usbh::{
    bus::HostBus,
    types::{DeviceAddress, SetupPacket, TransferType},
};

pub struct UsbhDefmt<INNER>
where
    INNER: HostBus,
{
    inner: INNER,
}

impl<INNER> UsbhDefmt<INNER>
where
    INNER: HostBus,
{
    pub fn new(inner: INNER) -> Self {
        Self { inner }
    }
}

impl<INNER> HostBus for UsbhDefmt<INNER>
where
    INNER: HostBus,
{
    #[trace]
    fn reset_controller(&mut self) {
        self.inner.reset_controller();
    }
    #[trace]
    fn reset_bus(&mut self) {
        self.inner.reset_bus();
    }
    #[trace]
    fn enable_sof(&mut self) {
        self.inner.enable_sof();
    }
    #[trace]
    fn sof_enabled(&self) -> bool {
        self.inner.sof_enabled()
    }
    fn set_recipient(
        &mut self,
        dev_addr: Option<DeviceAddress>,
        endpoint: u8,
        transfer_type: TransferType,
    ) {
        println!("enter set_recipient");
        self.inner.set_recipient(dev_addr, endpoint, transfer_type);
        println!("exit set_recipient");
    }
    #[trace]
    fn ls_preamble(&mut self, enable: bool) {
        self.inner.ls_preamble(enable);
    }
    #[trace]
    fn stop_transaction(&mut self) {
        self.inner.stop_transaction();
    }
    fn write_setup(&mut self, setup: SetupPacket) {
        println!("enter write_setup");
        self.inner.write_setup(setup);
        println!("exit write_setup");
    }
    #[trace]
    fn write_data_in(&mut self, length: u16, pid: bool) {
        self.inner.write_data_in(length, pid);
    }
    #[trace]
    fn prepare_data_out(&mut self, data: &[u8]) {
        self.inner.prepare_data_out(data);
    }
    #[trace]
    fn write_data_out_prepared(&mut self) {
        self.inner.write_data_out_prepared();
    }
    fn poll(&mut self) -> Option<usbh::bus::Event> {
        println!("enter poll");
        let result = self.inner.poll();
        println!("exit poll");
        result
    }
    #[trace]
    fn received_data(&self, length: usize) -> &[u8] {
        self.inner.received_data(length)
    }
    fn create_interrupt_pipe(
        &mut self,
        device_address: DeviceAddress,
        endpoint_number: u8,
        direction: UsbDirection,
        size: u16,
        interval: u8,
    ) -> Option<usbh::bus::InterruptPipe> {
        println!("enter create_interrupt_pipe");
        let result = self.inner.create_interrupt_pipe(
            device_address,
            endpoint_number,
            direction,
            size,
            interval,
        );
        println!("exit create_interrupt_pipe");
        result
    }
    #[trace]
    fn release_interrupt_pipe(&mut self, pipe_ref: u8) {
        self.inner.release_interrupt_pipe(pipe_ref);
    }
    #[trace]
    fn pipe_continue(&mut self, pipe_ref: u8) {
        self.inner.pipe_continue(pipe_ref);
    }
    #[trace]
    fn interrupt_on_sof(&mut self, enable: bool) {
        self.inner.interrupt_on_sof(enable);
    }
}
