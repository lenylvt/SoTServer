use std::{net::IpAddr, process::Command};

use colored::*;
use etherparse::{IpHeader, PacketHeaders};
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use winroute::{Route, RouteManager};

fn trouver_pid_sot(s: &System) -> Option<u32> {
    s.processes_by_name("SoTGame.exe").next().map(|process| process.pid().as_u32())
}

fn trouver_ports_sot(pid: u32) -> Vec<u16> {
    let pid_str = pid.to_string();

    let cmd = Command::new("netstat")
        .arg("-anop")
        .arg("udp")
        .output()
        .unwrap();

    // netstat peut contenir des caractères non-utf8
    let sortie_filtrée = cmd
        .stdout
        .iter()
        .filter(|c| c.is_ascii())
        .copied()
        .collect();

    String::from_utf8(sortie_filtrée)
        .unwrap()
        .lines()
        .filter(|line| line.contains(&pid_str))
        .map(|f| {
            let addr = f.split_whitespace().skip(1).next().unwrap();
            let port = addr.split(':').last().unwrap();
            port.parse::<u16>().unwrap()
        })
        .collect()
}

fn main() {
    println!("{}", "🔍 Vérification de l'installation de Npcap...".cyan().bold());
    unsafe {
        let essai_chargement_wpcap = libloading::Library::new("wpcap.dll");
        if essai_chargement_wpcap.is_err() {
            println!("{}", "⚠️  ERREUR  ⚠️".red().bold());
            println!("{}", "Npcap ne semble pas être installé sur votre système.".red());
            println!("{}", "Veuillez installer Npcap depuis:".yellow());
            println!("{}", "    https://npcap.com/dist/npcap-1.72.exe\n".cyan().underline());
            println!("{}", "⚠️  IMPORTANT: Activez l'option 'WinPcap API Compatibility' pendant l'installation  ⚠️".yellow().bold());
            println!("\n{}", "Continuer quand même? (oui/non):".green());
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if !(input == "o" || input == "oui") {
                std::process::exit(1);
            }
        }
    }

    // Attendre que Sea of Thieves soit lancé
    println!("{}", "⏳ En attente du lancement de Sea of Thieves...".cyan().bold());
    let mut s = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));

    let sot_pid = loop {
        if let Some(pid) = trouver_pid_sot(&s) {
            break pid;
        }
        s.refresh_processes();
    };

    println!("{} {}", "✅ Sea of Thieves détecté! PID:".green().bold(), sot_pid.to_string().yellow());

    let devices = pcap::Device::list().unwrap();
    let adaptateur_auto = devices.iter().find(|d| {
        d.addresses.iter().any(|addr| {
            if let IpAddr::V4(addr) = addr.addr {
                addr.octets()[0] == 192 && addr.octets()[1] == 168
            } else {
                false
            }
        })
    });

    let dev = match adaptateur_auto {
        Some(d) => d.clone(),
        None => {
            println!("{}", "🔌 Impossible de déterminer automatiquement l'adaptateur réseau.".yellow().bold());
            println!("{}", "Veuillez sélectionner manuellement:".yellow());

            let devices = pcap::Device::list().expect("échec de recherche d'appareils");
            let mut i = 1;

            for device in devices.clone() {
                println!(
                    "    {}. {}",
                    i.to_string().cyan(),
                    device.desc.clone().unwrap_or(device.name.clone()).white()
                );
                i += 1;
            }

            println!("{}", "Sélectionnez votre carte WiFi, Ethernet ou VPN: ".green());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let n = input.trim().parse::<usize>().unwrap() - 1;

            (&devices[n]).clone()
        }
    };

    let mut cap = pcap::Capture::from_device(dev)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    let route_manager = RouteManager::new().unwrap();
    let void_ip = "0.0.0.0".parse().unwrap();

    println!("{}", "🌊 À quel serveur souhaitez-vous vous connecter? (ex: 20.213.146.107:30618)".cyan().bold());
    println!("{}", "    Entrez 'idk' pour simplement afficher le serveur auquel vous êtes connecté.".italic());
    let mut cible = String::new();
    std::io::stdin().read_line(&mut cible).unwrap();
    let cible = cible.trim();

    if cible == "idk" {
        println!("{}", "🔍 Affichage du serveur en cours...".green());
    } else {
        println!("{} {}", "🎯 Cible:".green(), cible.yellow().bold());
    }

    println!("{}", "⏳ En attente de connexion à un serveur Sea of Thieves...".cyan().bold());

    // Analyse des paquets UDP
    loop {
        if let Ok(paquet_brut) = cap.next_packet() {
            if let Ok(paquet) = PacketHeaders::from_ethernet_slice(paquet_brut.data) {
                if let Some(IpHeader::Version4(ipv4, _)) = paquet.ip {
                    if let Some(transport) = paquet.transport {
                        if let Some(udp) = transport.udp() {
                            if udp.destination_port == 3075 || udp.destination_port == 30005 {
                                continue;
                            }

                            if trouver_ports_sot(sot_pid).contains(&udp.source_port) {
                                let ip = ipv4.destination.map(|c| c.to_string()).join(".");

                                if cible == "idk" {
                                    println!("{} {}:{}", "🏝️  Serveur actuel:".green().bold(), ip.yellow(), udp.destination_port.to_string().yellow());
                                    println!("{}", "   Appuyez sur Entrée pour vérifier à nouveau.".italic());
                                    std::io::stdin().read_line(&mut String::new()).unwrap();
                                    continue;
                                }

                                if format!("{}:{}", ip, udp.destination_port) != cible {
                                    println!(
                                        "{} {}:{}",
                                        "❌ ÉCHEC".red().bold(),
                                        ip.yellow(),
                                        udp.destination_port.to_string().yellow()
                                    );
                                } else {
                                    println!(
                                        "{} {}:{}",
                                        "✅ SUCCÈS".green().bold(),
                                        ip.yellow(),
                                        udp.destination_port.to_string().yellow()
                                    );
                                    std::io::stdin().read_line(&mut String::new()).unwrap();
                                    break;
                                }

                                let route_bloquante = Route::new(ip.parse().unwrap(), 32).gateway(void_ip);

                                // Ajout de la route
                                if let Err(e) = route_manager.add_route(&route_bloquante) {
                                    println!(
                                        "{} {}:{} - {}",
                                        "⚠️  Erreur d'ajout de route pour:".red().bold(),
                                        ip.yellow(),
                                        udp.destination_port.to_string().yellow(),
                                        e
                                    );
                                } else {
                                    println!("{}", "⚠️  Répondez NON à 'Voulez-vous rejoindre votre session précédente?'".yellow().bold());
                                    println!("{}", "   Puis appuyez sur Entrée ici.".italic());
                                    std::io::stdin().read_line(&mut String::new()).unwrap();
                                }

                                println!("{} {}", "🔓 Déblocage de".green(), ip.yellow());

                                // Suppression de la route (route_manager.delete_route ne fonctionne pas correctement)
                                let statut = Command::new("route")
                                    .arg("delete")
                                    .arg(ip)
                                    .status()
                                    .unwrap();
                                if !statut.success() {
                                    println!("{}", "❌ Échec de suppression de la route.".red());
                                }

                                println!("{}", "🚢 Essayez de lever l'ancre à nouveau.".cyan().bold());
                            }
                        }
                    }
                }
            }
        }
    }
}
