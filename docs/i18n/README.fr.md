# monadclaw

Un framework d'agent IA personnel minimaliste, modulaire et extensible, écrit en Rust.
Projet personnel — non destiné à une utilisation en production.

**Autres langues :** [English](../../README.md) · [中文](README.zh.md)

---

## Démarrage rapide

### Prérequis

- Rust (stable) — [rustup.rs](https://rustup.rs)
- Node.js 18+ — pour le dashboard
- Une clé API d'un fournisseur LLM supporté (ex. [OpenRouter](https://openrouter.ai/keys))

### 1. Configuration

Créer `config.toml` à la racine du projet (ou dans `~/.config/monadclaw/config.toml`) :

```toml
active_provider = "openrouter"

[providers.openrouter]
model = "openai/gpt-4o-mini"
api_key_env = "OPENROUTER_API_KEY"
base_url = "https://openrouter.ai/api/v1/"
```

Créer un fichier `.env` à la racine du projet :

```env
OPENROUTER_API_KEY=sk-or-v1-...
```

### 2. Démarrer le backend

```bash
source .env && MONADCLAW_CONFIG=./config.toml cargo run
# Serveur disponible sur http://0.0.0.0:3000
```

### 3. Démarrer le dashboard

```bash
cd dashboard
npm install
npm run dev
# Dashboard sur http://localhost:5173
```

---

## Authentification

Monadclaw utilise un **modèle d'accès à trois niveaux** basé sur l'origine de la connexion et la présence d'un mot de passe.

| Connexion | Mot de passe défini ? | Résultat |
|----------|----------------------|----------|
| Local (loopback) | Non | ✅ Accès libre — aucun identifiant requis |
| Local (loopback) | Oui | 🔑 Bearer token requis |
| Distant | Non | ❌ 403 Forbidden |
| Distant | Oui | 🔑 Bearer token requis |

### Définir un mot de passe

Ajouter `dashboard_password` dans `config.toml` et redémarrer le serveur :

```toml
dashboard_password = "votre-mot-de-passe"
```

Le dashboard affichera une page de connexion. Le token est stocké dans `localStorage` sans expiration. Vider le stockage du navigateur pour se déconnecter.

### Accès distant

L'accès distant est **bloqué par défaut** en l'absence de mot de passe — mesure de sécurité intentionnelle.
Pour l'activer, définir `dashboard_password` dans la configuration.

> Voir [docs/auth.md](../auth.md) pour la politique d'authentification complète.

---

## Structure du projet

```
monadclaw/
├── apps/server/        # Point d'entrée binaire (serveur HTTP Axum)
├── crates/
│   ├── api/            # Routeur Axum, routes, middleware
│   ├── chat/           # Types de messages de chat
│   ├── config/         # Chargement de la configuration TOML
│   └── providers/      # Abstraction des fournisseurs LLM (genai)
├── dashboard/          # Dashboard React 19 + TypeScript
├── docs/               # Spécifications et documentation internes
└── config.toml         # Configuration locale (ignorée par git)
```

---

## Feuille de route

| Fonctionnalité | État |
|---------------|------|
| Chargement config TOML + résolution variables d'environnement | ✅ Terminé |
| Abstraction fournisseurs LLM (genai) | ✅ Terminé |
| Endpoints compatibles OpenAI (OpenRouter, Kimi, etc.) | ✅ Terminé |
| API chat en streaming (`POST /api/v1/chat`) | ✅ Terminé |
| API statut (`GET /api/v1/status`) | ✅ Terminé |
| Serveur HTTP Axum avec CORS | ✅ Terminé |
| Dashboard React — shell, barre latérale, navigation | ✅ Terminé |
| Page chat avec réponses en streaming | ✅ Terminé |
| Middleware d'authentification trois niveaux | ✅ Terminé |
| Page de connexion + garde de route | ✅ Terminé |
| Boucle agent (appels d'outils, raisonnement multi-étapes) | 🔄 Prévu |
| Mémoire court terme (fenêtre de conversation) | 🔄 Prévu |
| Mémoire long terme (stockage persistant) | 🔄 Prévu |
| Interface bot Discord | 🔄 Prévu |
| Fournisseurs LLM multiples (Anthropic, Gemini, etc.) | 🔄 Prévu |
| Éditeur de configuration dans le dashboard | 🔄 Prévu |
| Historique des sessions | 🔄 Prévu |
| Suivi d'utilisation | 🔄 Prévu |
| Visualiseur de logs | 🔄 Prévu |
| Tâches planifiées (cron) | 🔄 Prévu |
| Système de skills / extensions | 🔄 Prévu |

---

## Licence

MIT
