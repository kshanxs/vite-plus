# options_before_command_local

## `vp --filter app-a run hello`

--filter before the command

```
~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp -w run hello`

-w before the command

```
$ node -e "console.log('hello from the root')" ⊘ cache disabled
hello from the root
```

## `vp --nope run hello`

an option the command does not have

**Exit code:** 2

```
error: Unexpected argument '--nope'
```

## `vp -w build`

tool commands only take options after the command

**Exit code:** 2

```
error: Unexpected argument '-w'
```
