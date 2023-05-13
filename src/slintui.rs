use std::{os::unix::process::CommandExt, rc::Rc};

use slint::VecModel;

use crate::{
    login::{login, LoginResult},
    xdginfos::DESKTOPS,
};

slint::include_modules!();

pub fn greet_ui() -> anyhow::Result<()> {
    let ui = AppWindow::new()?;

    let systeminfos = SystemInfos::get(&ui);
    let mut commands = Vec::new();
    let mut des = Vec::new();
    for wmss in &*DESKTOPS {
        commands.push(wmss.exec.clone().into());
        des.push(wmss.name.clone().into());
    }
    systeminfos.set_wms(Rc::new(VecModel::from(des)).into());
    systeminfos.set_wm_command(Rc::new(VecModel::from(commands)).into());
    let ui_handle = ui.as_weak();
    let ui_handle2 = ui.as_weak();

    ui.on_Login(move |user, passward, cmd| {
        let ui_inner = ui_handle.unwrap();
        let cmd = cmd
            .split(' ')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|cmd| cmd.to_string())
            .collect();
        match login(user.into(), passward.into(), cmd) {
            Ok(LoginResult::Success) => {
                ui_inner.invoke_SetMessage("info".into(), "Logininto".into());
                let _ = slint::quit_event_loop();
            }
            Ok(LoginResult::Failure(message)) => {
                ui_inner.invoke_SetMessage("Warning".into(), message.into());
            }
            Err(e) => {
                println!("error");
                ui_inner.invoke_SetMessage("Warning".into(), e.to_string().into());
            }
        };
    });

    ui.on_Shutdown(move || {
        let ui_inner = ui_handle2.unwrap();
        let shutdown = std::process::Command::new("shutdown").arg("now").exec();
        println!("shutdown failed");
        ui_inner.invoke_SetMessage("Warning".into(), shutdown.to_string().into());
    });

    ui.run()?;
    Ok(())
}
