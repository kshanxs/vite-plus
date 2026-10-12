# options_before_command

## `vp --filter app-a run hello`

--filter before the command

```
VITE+ - The Unified Toolchain for the Web

~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp -w run hello`

-w before the command

```
VITE+ - The Unified Toolchain for the Web

$ node -e "console.log('hello from the root')" ⊘ cache disabled
hello from the root
```

## `vp -F app-b exec -- node -e console.log(process.env.VP_PACKAGE_NAME)`

-F before exec

```
VITE+ - The Unified Toolchain for the Web

app-b
```

## `vp --ignore-depends-on -F app-a run hello`

any option of the command, not only package selection

```
VITE+ - The Unified Toolchain for the Web

~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp --filter run run hello`

an option value that looks like a command

```
VITE+ - The Unified Toolchain for the Web

No packages matched the filter: run
```

## `vp run nested`

the same order inside a script

```
VITE+ - The Unified Toolchain for the Web

~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp --nope run hello`

an option the command does not have

**Exit code:** 2

```
VITE+ - The Unified Toolchain for the Web

error: Unexpected argument '--nope'
```

## `vp -r fmt`

tool commands only take options after the command

**Exit code:** 2

```
VITE+ - The Unified Toolchain for the Web

error: Unexpected argument '-r'
```
