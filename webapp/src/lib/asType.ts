// TypeScript doesn't inline typed object literals yet.
// See: https://github.com/microsoft/TypeScript/issues/7481

// The 'as' keyword performs type assertion, which basically tells the compiler
// to ignore type checks:

// interface IFoo { foo: string; }
// interface IBar { bar?: IFoo; }
// const x: IBar = {};
// x.bar = { lol: 42 } as IFoo;  // <- BAD, no error is thrown. :(

// One way to enforce type checks is typed variables:
// const y: IFoo = { foo: '42' };  // <- GOOD, type checked here
// x.bar = y;

// TypeScript doesn't have a built-in way to define object literals with
// enforced type checks. A common way to circumvent this shortcoming is
// the 'asType' function:
// x.bar = asType<IFoo>({ lol: 42 });  // <- GOOD, throws error, no temp variable.

export function asType<T>(value: T): T {
  return value;
}
