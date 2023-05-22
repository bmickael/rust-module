# ___FORMATION  AU LANGAGE DE PROGRAMMATION RUST & WEBASM___

___

Bienvenue dans cette formation ! Contrairement à d'autres écoles qui préconisent l'utilisation massive de slides, j'ai préféré créé une image Docker ainsi que des notebooks Jupyter avec le noyau Rust EvCxR pour rendre cette formation plus interactive et vous éviter d'avoir à installer Rust sur vos propres machine. Vous aurez aussi la possibilité d'annoter voir de complètement modifier le cours.

Vous pouvez trouver la documentation du noyau EvCxR à [l'adresse suivante](https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/ "REPL AND JUPYTER").

- Grâce à ce noyau, il sera possible d'exécuter du code rust.


```Rust
let a = 2;
let b = 2;
println!("{} et {} font {}", a, b, a + b + 1);
```

    2 et 2 font 5


- L'interpréteur offre quelques instructions spéciales comme **:vars** qui permet de lister les variables.


```Rust
:vars
```




<table><tr><th>Variable</th><th>Type</th></tr><tr><td>a</td><td>i32</td><tr><tr><td>b</td><td>i32</td><tr></table>




```Rust
let s: String = "Banane".to_owned();
```

***Il est aussi possible d'ouvrir un terminal pour se rendre aux projets contenus dans l'image Docker, les compiler pour les exécuter.***
![TERMINAL_1](notebooks/000%20Readme%20pictures/new_terminal.png)

***L'image Docker est une debian minimale, mais le terminal suffit largement pour les besoins de la formation.***  
![TERMINAL_2](notebooks/000%20Readme%20pictures/terminal_example.png)
![FISH](notebooks/000%20Readme%20pictures/fishes.png)

***Si vous hésitez quant à l'éditeur à utiliser pour la formation, sachez que `emacs` est installé dans le container avec `evil mode` pour les raccourcis VIM ainsi que `rust-analyzer` qui donne les erreurs. (Et pour les petits besoins, `nano` est installé aussi). Pour le lancer, il suffit de taper `emacs` ou `emacs -nw` selon le contexte.***
![EMACS](notebooks/000%20Readme%20pictures/emacs.png)

***Evidemment, vous avez aussi la possibilité d'accéder directement au container docker via `docker compose exec jupyter-server bash` dans le dossier principal du depot ou bien meme `docker exec -it [CONTAINER_ID] bash`.***
![DUAL_TERM](notebooks/000%20Readme%20pictures/dual_term.png)

***Enfin sachez que si vous avez un hôte compatible X11, il est possible d'utiliser Visual Studio Code en mode graphique depuis le container. Pour des raisons de sécurité, cette fonctionnalité n'est pas activée par défaut, alors, afin de pouvoir donner au container le contrôle du serveur X, il faut modifier le fichier `docker-compose.yml` en decommentant la ligne `# - "/tmp/.X11-unix:/tmp/.X11-unix"`. Enfin, relancer le container avec la commande `docker compose up` et dans un autre invite de commande, écrire `docker compose exec jupyter-server bash code`. Si votre hôte est compatible, xcode se lancera. Notez enfin que vous pouvez lancer de la même façon emacs en mode graphique en remplaçant `code` par `emacs`.***
![VS_CODE](notebooks/000%20Readme%20pictures/vscode.png)

> ***Tout le contenu de la formation se trouve dans le dossier rust-module du dépôt de votre machine hôte, vous pouvez directement accéder aux fichiers et les modifier depuis votre éditeur favori (comme VSCode par exemple).***

> ___IMPORTANT___
>  
> *l'UID et le GID de l'utilisateur du container docker est 1000:1000, il est possible que vous ayez à modifier le uid/gid des dossiers `./rust-module` et `./hidden.directory.config` en fonction grâce à la commande chown: `sudo chown -R 1000:1000 rust-module hidden.directory.config`. En dernier recours, l'umask utilisé pour la génération des fichiers depuis le container devra être changé en 0000. Cela se fait dans les fichiers `./hidden.directory.config/bash/bashrc` et `./hidden.directory.config/container-init.sh`*.

- Enfin, il y aura des solutions cachées qui pourront être débloquées facilement.

**Comment A-ton découvert la radioactivité ?**


```Rust
println!("Le 26 février 1896, le physicien Henri Becquerel a enfermé par hasard, dans un tiroir des cailloux d'uranium avec des plaques photographiques. Quatre jours plus tard, il a découvert en développant les plaques photographiques, la silhouette des cailloux d'uranium.");

```

    Le 26 février 1896, le physicien Henri Becquerel a enfermé par hasard, dans un tiroir des cailloux d'uranium avec des plaques photographiques. Quatre jours plus tard, il a découvert en développant les plaques photographiques, la silhouette des cailloux d'uranium.


*Pour en savoir plus sur le kernel RUST de Jupyter, rendez vous [le dépot github officiel](https://github.com/hgfkeep/rust-jupyter/blob/master/rust-jupyter-example.ipynb "Tour of the EvCxR Jupyter Kernel").*

>**ATTENTION, LES CHAPITRES 009 A 014 SONT PUREMENT INFORMATIFS ET NE FONT PAS PARTIE DE LA FORMATION RUST AVEC WEBASSEMBLY. D'AUTRES NOTIONS COMME LES TRAITS OBJET, LES FONCTIONS CONSTANTES, LES SUPERTRAITS, L'UNSAFE RUST ET LES MACROS PROCEDURALES NE SONT PAS PRESENTES DU TOUT PUISQUE TROP AVANCÉES.**
