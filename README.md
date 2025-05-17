# 🌊 SoT Server Finder

> Trouvez facilement sur quel serveur Sea of Thieves vous êtes connecté et rejoignez vos amis sur le même serveur.

## 📋 Prérequis

- Windows 10/11
- [Npcap](https://npcap.com/dist/npcap-1.72.exe) (avec mode de compatibilité WinPCap activé)
- Droits administrateur

## 🚀 Installation

### Téléchargement
- [Télécharger le fichier exécutable](https://github.com/xxcodianxx/sot-server-finder/releases/download/0.1.0/sot-server-finder.exe)

### Compilation
```bash
cargo build --release
```
> L'exécutable sera disponible dans `target/release/sot-server-finder.exe`

## 🎮 Utilisation

1. **Lancez le programme en mode administrateur**
2. **Démarrez Sea of Thieves**
3. **Sélectionnez votre mode** :
   - `idk` pour afficher le serveur actuel
   - Entrez une adresse IP pour rejoindre un serveur spécifique

### 🔄 Changer de serveur

1. Définissez le serveur cible (obtenu d'un ami)
2. Mettez les voiles dans Sea of Thieves
3. Le programme bloquera les connexions aux serveurs non désirés
4. À l'échec de connexion, répondez **NON** à "Voulez-vous rejoindre votre session précédente?"
5. Appuyez sur Entrée pour débloquer et réessayer
6. Répétez jusqu'à atteindre le bon serveur

## ⚠️ Remarques

- Plus vous avez d'amis qui font la même chose, plus vos chances augmentent!
- Fonctionne uniquement sur Windows
- Le script de compilation télécharge automatiquement le SDK Npcap 1.13

## 🔗 Voir aussi
- [SeaOfEase](https://github.com/Saeryhz/SeaOfEase)