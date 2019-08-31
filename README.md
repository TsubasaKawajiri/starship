<p align="center">
  <br />
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt"
  />
</p>
<p align="center">
  <a href="https://repology.org/project/starship/versions">
    <img src="https://repology.org/badge/tiny-repos/starship.svg" alt="Packaging status">
  </a>
  <a href="https://crates.io/crates/starship">
    <img src="https://badgen.net/crates/v/starship" alt="Crates.io version" />
  </a>
  <a href="https://dev.azure.com/starship-control/starship/_build">
    <img
      src="https://badgen.net/azure-pipelines/starship-control/starship/Starship%20Test%20Suite"
      alt="Azure Pipelines Build Status"
    />
  </a>
  <a href="#contributors">
    <img
      src="https://badgen.net/badge/all%20contributors/14/orange"
      alt="All Contributors"
    />
  </a>
  <a href="https://discord.gg/8Jzqu3T">
    <img
      src="https://badgen.net/badge/chat/on%20discord/7289da"
      alt="Chat on Discord"
    />
  </a>
</p>

## 📄Changes from base Starship
- Add datetime module.
  - display local datetime.
  - configurable display format.
  
- modify battery module.
  - configurable display threshold.
  - display battery cycles.
  
## 📚Additional Configures
```
[battery]
threshold = 10.0    // 0.0 ~ 100.0
show_cylce = false  // true or false

[datetime]
disable = false     // true or false
format = "%Y/%m/%d" // year:%Y, month:%m, day:%d, hour:%H, minute:%M, second:%S, timezone:%Z
```
  

## 🚀
thank you starship.
I've fun starship!

<p align="center">
    <br>
    <img width="100" src="media/icon.png" alt="Starship rocket icon">
</p>

## 📝 License

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br>
This project is [ISC](https://github.com/starship/starship/blob/master/LICENSE) licensed.
