# Directory Structure

```
├── 3rd
|   ├──protoc //Object protobuf processing
├── .eslintignore // Configuration file to be ignored by eslint
├── .eslintrc.js // eslint configuration file
├── package.json // cyfs package and upload the dependent node library
├── prettier.config.js // prettier configuration file
├── readme.md // Description file
├── tsconfig.json // typescript configuration file
├── webpack.config.js // webpack configuration file
├── readme.md
└── src
    ├── apis // interface file
    ├── assets // static resource file
    ├── components // component file
    ├── constants // constants file
    ├── i18n // internationalized language pack
    ├── stores // global state file
    ├── pages // page file
    ├── routers // routing files
    ├── styles // style file
    ├── types // types file
    └── utils // utility file
```

# Devlopment

## run dev

```sh
yarn dev
```

## lint check

```sh
yarn lint
```

## lint fix

```sh
yarn lint-fix
```
