------------------------- MODULE VNAGY_Canonical_Spec -------------------------
EXTENDS Naturals, Sequences, TLC

CONSTANTS ConfigFormats, MaxSteps

VARIABLES 
    configFormat,
    globalStateActive,
    actionTriggered,
    symbolicState,
    executionScope,
    stepCount

vars == <<configFormat, globalStateActive, actionTriggered, symbolicState, executionScope, stepCount>>

Init ==
    /\ configFormat = "TOML"
    /\ globalStateActive = FALSE
    /\ actionTriggered = FALSE
    /\ symbolicState = "Idle"
    /\ executionScope = "Static"
    /\ stepCount = 0

IngestConfiguration(fmt) ==
    /\ stepCount < MaxSteps
    /\ fmt \in ConfigFormats
    /\ configFormat' = fmt
    /\ executionScope' = "Static"
    /\ stepCount' = stepCount + 10
    /\ UNCHANGED <<globalStateActive, actionTriggered, symbolicState>>

EvaluateSymbolicState(token) ==
    /\ stepCount < MaxSteps
    /\ token \in {"Safe", "Warning", "Critical", "Invalid"}
    /\ symbolicState' = token
    /\ actionTriggered' = FALSE
    /\ stepCount' = stepCount + 10
    /\ UNCHANGED <<configFormat, globalStateActive, executionScope>>

Next ==
    \/ \E fmt \in ConfigFormats : IngestConfiguration(fmt)
    \/ \E token \in {"Safe", "Warning", "Critical", "Invalid"} : EvaluateSymbolicState(token)

Spec == Init /\ [][Next]_vars

\* -------------------------------------------------------------------------------
\* NORMATIVE SAFETY INVARIANTS
\* -------------------------------------------------------------------------------

Inv_ConfigurationStandardization == 
    configFormat \in {"TOML"}

Inv_NoGlobalState == 
    globalStateActive = FALSE

Inv_NonOperationalScoring == 
    actionTriggered = FALSE

=============================================================================
