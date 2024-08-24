#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{peripherals::USART3, usart::Uart};
use embassy_time::{with_timeout, Timer};
use lin_bus::{Frame, PID};
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::{bind_interrupts, usart};


bind_interrupts!(struct UARTIRqs {
    USART3 => usart::InterruptHandler<USART3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let config = {
        let mut config = usart::Config::default();
        config.baudrate = 19200;
        config.extended_feature = Some(usart::ExtendedFeature::LIN);
        config
    };

    let (mut tx, mut rx) = Uart::new(
        p.USART3,
        p.PB11,
        p.PB10,
        UARTIRqs,
        p.DMA1_CH2,
        p.DMA1_CH3,
        config,
    ).unwrap().split();

    let mut dma_buf = [0; 32];
    let mut rx = rx.into_ring_buffered(&mut dma_buf);

    let Tbit = 1.0 / 19200.0;
    let T_header_nominal = 34.0 * Tbit;
    let T_header_maximum = 1.4 * T_header_nominal;
    let T_response_nominal = 10.0 * (8.0 + 1.0) * Tbit;
    let T_response_maximum = 1.4 * T_response_nominal;


    
    loop {
        // get framming error
        let mut buf = [0; 1];
        let res = rx.read(&mut buf).await;
        info!("{} {:x}", res, buf);
        continue;

/*

       // get framming error
       let mut buf = [0; 1];
       let res = uart.read(&mut buf).await;
       if res != Err(embassy_stm32::usart::Error::Framing) {
           info!("Expected framing error!, got: {} {:x}", res, buf);
           continue;
       }

       info!("okay?");

       let mut hdr = [0u8; 2];
       match with_timeout(
           embassy_time::Duration::from_micros((T_header_nominal * 1_000_000.0) as u64),
           uart.read(&mut hdr),
       )
       .await
       {
           Ok(Ok(())) => {
               if hdr[0] != 0x55 {
                   info!("Hdr not starting with 0x55, but {}", hdr[0]);
                   continue;
               }
           }
           Ok(Err(err)) => {
               info!("Hdr error {}", err);
               continue;
           }
           Err(f) => {
               info!("Hdr timeout");
               continue;
           }
       };

       let pid = PID::new(hdr[1]);
       if pid.get_id() == 0x32 {
           let resp =
               lin_bus::Frame::from_data(pid, &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
           uart.write(resp.get_data_with_checksum()).await.unwrap();
           info!("responding! {:x}", pid.get_id());
           let mut buf = [0; 9];
           uart.read(&mut buf).await.unwrap();
           info!("read back: {:x}", buf);
       } else {
           let mut buf = [0; 9];
           match with_timeout(
               embassy_time::Duration::from_micros((T_response_maximum * 1_000_000.0) as u64),
               uart.read(&mut buf),
           )
           .await
           {
               Ok(Ok(())) | Err(_) => {
                   let checksum = buf[buf.len() - 1];
                   let frame = lin_bus::Frame::from_data(pid, &buf[..buf.len() - 1]);
                   info!("PID {} {:x}", frame.get_pid().get_id(), frame.get_data());
               }
               Ok(Err(err)) => {
                   info!("Hdr error {}", err);
                   continue;
               }
           };
        }

*/


        /*
        info!("LIN frame sent");
        let pid = PID::from_id(0x8);
        let f = Frame::from_data(pid, &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        uart.send_break();
        uart.write(&[0x55, pid.get()]).await.unwrap();
        uart.write(&f.get_data_with_checksum()).await.unwrap();
        */

        Timer::after_millis(300).await;
    }

}
