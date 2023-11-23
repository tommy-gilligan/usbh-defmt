#![no_std]

use defmt::trace;
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
    fn reset_controller(&mut self) {
        trace!("reset_controller start");
        self.inner.reset_controller();
        trace!("reset_controller end");
    }
    fn reset_bus(&mut self) {
        trace!("reset_bus start");
        self.inner.reset_bus();
        trace!("reset_bus end");
    }
    fn enable_sof(&mut self) {
        trace!("enable_sof start");
        self.inner.enable_sof();
        trace!("enable_sof end");
    }
    fn sof_enabled(&self) -> bool {
        trace!("sof_enabled start");
        let result = self.inner.sof_enabled();
        trace!("sof_enabled end {:?}", result);
        result
    }
    fn set_recipient(
        &mut self,
        dev_addr: Option<DeviceAddress>,
        endpoint: u8,
        transfer_type: TransferType,
    ) {
        trace!(
            "set_recipient start dev_addr {:?} endpoint {:?}",
            dev_addr,
            endpoint,
        );
        self.inner.set_recipient(dev_addr, endpoint, transfer_type);
        trace!(
            "set_recipient end dev_addr {:?} endpoint {:?}",
            dev_addr,
            endpoint,
        );
    }
    fn ls_preamble(&mut self, enable: bool) {
        trace!("ls_preamble start dev_addr {:?}", enable);
        self.inner.ls_preamble(enable);
        trace!("ls_preamble end dev_addr {:?}", enable);
    }
    fn stop_transaction(&mut self) {
        trace!("stop_transaction start");
        self.inner.stop_transaction();
        trace!("stop_transaction end");
    }
    fn write_setup(&mut self, setup: SetupPacket) {
        trace!("write_setup start");
        self.inner.write_setup(setup);
        trace!("write_setup end");
    }
    fn write_data_in(&mut self, length: u16, pid: bool) {
        trace!("write_data_in start length {:?} pid {:?}", length, pid);
        self.inner.write_data_in(length, pid);
        trace!("write_data_in end length {:?} pid {:?}", length, pid);
    }
    fn prepare_data_out(&mut self, data: &[u8]) {
        trace!("prepare_data_out start data {:?}", data);
        self.inner.prepare_data_out(data);
        trace!("prepare_data_out end data {:?}", data);
    }
    fn write_data_out_prepared(&mut self) {
        trace!("write_data_out_prepared start");
        self.inner.write_data_out_prepared();
        trace!("write_data_out_prepared end");
    }
    fn poll(&mut self) -> Option<usbh::bus::Event> {
        trace!("poll start");
        let result = self.inner.poll();
        trace!("poll end {:?}", result);
        result
    }
    fn received_data(&self, length: usize) -> &[u8] {
        trace!("received_data start {:?}", length);
        let result = self.inner.received_data(length);
        trace!("received_data end length {:?} {:?}", length, result);
        result
    }
    fn create_interrupt_pipe(
        &mut self,
        device_address: DeviceAddress,
        endpoint_number: u8,
        direction: UsbDirection,
        size: u16,
        interval: u8,
    ) -> Option<usbh::bus::InterruptPipe> {
        trace!(
            "create_interrupt_pipe start device_address {:?} endpoint_number {:?} direction {:?} size {:?} interval {:?}",
            device_address,
            endpoint_number,
            direction,
            size,
            interval,
        );
        let result = self.inner.create_interrupt_pipe(
            device_address,
            endpoint_number,
            direction,
            size,
            interval,
        );
        trace!(
            "create_interrupt_pipe end device_address {:?} endpoint_number {:?} direction {:?} size {:?} interval {:?}",
            device_address,
            endpoint_number,
            direction,
            size,
            interval,
        );
        result
    }
    fn release_interrupt_pipe(&mut self, pipe_ref: u8) {
        trace!("release_interrupt_pipe start {:?}", pipe_ref);
        self.inner.release_interrupt_pipe(pipe_ref);
        trace!("release_interrupt_pipe end {:?}", pipe_ref);
    }
    fn pipe_continue(&mut self, pipe_ref: u8) {
        trace!("pipe_continue start {:?}", pipe_ref);
        self.inner.pipe_continue(pipe_ref);
        trace!("pipe_continue end {:?}", pipe_ref);
    }
    fn interrupt_on_sof(&mut self, enable: bool) {
        trace!("interrupt_on_sof start {:?}", enable);
        self.inner.interrupt_on_sof(enable);
        trace!("interrupt_on_sof end {:?}", enable);
    }
}
