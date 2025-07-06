# ADR-0001: Standardize Terminology for Felled Trees to 'Stump'

- Date: 2025-07-07
- Status: Accepted

## Context

In our application's data model, we track the status of trees. One of the key states represents a tree that has been cut down, leaving only the base. Initially, this state was implemented in the code and stored in the database with the value "stomp".

During a recent review, it was identified that "stomp" is an incorrect term for this state. The verb "stomp" means to tread or trample heavily. The correct noun for the remaining base of a felled tree is "stump". This terminological inaccuracy could lead to confusion for new developers, data analysts, and stakeholders interacting with our system.

## Decision

We will standardize the terminology across the entire application to use the correct term, "stump".

This change involves:

1. Codebase: All variables, constants, enums, and documentation that previously referred to "stomp" will be refactored to use "stump".

2. Database: A data migration script will be executed on all environments (development, staging, and production) to update the `status` column in the `trees` table. All existing records with the value "stomp" will be changed to "stump".

3. API & UI: Any API responses or UI components that exposed this term will be updated to reflect the change.

## Consequences

Positive:

* Improved Clarity: The application's domain language is now accurate and intuitive, reducing the risk of misunderstanding.

* Enhanced Maintainability: Future developers will have a clearer understanding of the data model without historical ambiguity.

* Data Consistency: The data is now clean and uses a standardized, correct term for this state.

Negative:

* Effort: Required a development effort to refactor the code and create a reliable data migration script.

* Deployment Coordination: The deployment required careful coordination to ensure the code changes and data migration were applied together to avoid inconsistencies.

* External Dependencies: Any external consumers of our API or data (if any) needed to be notified of the change in this value.
