[Exposed=Window]
interface NavigateEvent : Event {
  constructor(DOMString type, NavigateEventInit eventInitDict);

  readonly attribute NavigationType navigationType;
  readonly attribute NavigationDestination destination;
  readonly attribute boolean canIntercept;
  readonly attribute boolean userInitiated;
  readonly attribute boolean hashChange;
  readonly attribute AbortSignal signal;
  readonly attribute FormData? formData;
  readonly attribute DOMString? downloadRequest;
  readonly attribute any info;
  readonly attribute boolean hasUAVisualTransition;

  undefined intercept(optional NavigationInterceptOptions options = {});
  undefined scroll();
};

dictionary NavigateEventInit : EventInit {
  NavigationType navigationType = "push";
  required NavigationDestination destination;
  boolean canIntercept = false;
  boolean userInitiated = false;
  boolean hashChange = false;
  required AbortSignal signal;
  FormData? formData = null;
  DOMString? downloadRequest = null;
  any info;
  boolean hasUAVisualTransition = false;
};

dictionary NavigationInterceptOptions {
  NavigationInterceptHandler handler;
  NavigationFocusReset focusReset;
  NavigationScrollBehavior scroll;
};

enum NavigationFocusReset {
  "after-transition",
  "manual"
};

enum NavigationScrollBehavior {
  "after-transition",
  "manual"
};

callback NavigationInterceptHandler = Promise<undefined> ();