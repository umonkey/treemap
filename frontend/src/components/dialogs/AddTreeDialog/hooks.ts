// Global imports.
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { toast } from "react-hot-toast";

// Project imports.
import { treeMapService } from "@/services/api";
import { IAddTreeRequest, ILatLng } from "@/types";
import { routes } from "@/utils";

interface IProps {
  center: ILatLng;
}

export const useAddTreeDialog = (props: IProps) => {
  const [species, setSpecies] = useState<string>("");
  const [height, setHeight] = useState<number>(0);
  const [circumference, setCircumference] = useState<number>(0);
  const [diameter, setDiameter] = useState<number>(0);
  const [state, setState] = useState<string>("healthy");
  const [notes, setNotes] = useState<string>("");
  const [busy, setBusy] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const navigate = useNavigate();

  const isSaveEnabled = (): boolean => {
    if (species.length === 0) {
      return false;
    }

    if (busy) {
      return false;
    }

    return true;
  };

  const handleNotesChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNotes(event.target.value);
  };

  const handleHeightChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setHeight(parseFloat(event.target.value));
  };

  const handleCircumferenceChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setCircumference(parseFloat(event.target.value));
  };

  const handleDiameterChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setDiameter(parseFloat(event.target.value));
  };

  const handleSaveClick = async () => {
    if (!species) {
      console.error("species not set, cannot add tree.");
      return;
    }

    setBusy(true);
    setError(null);

    try {
      const res = await treeMapService.addMarker({
        lat: props.center.lat,
        lon: props.center.lon,
        species,
        height,
        circumference,
        diameter,
        state,
        notes: notes || null,
      } as IAddTreeRequest);

      console.debug(`Tree added with id ${res.id}, showing details page.`);

      toast.success("Tree added successfully.");

      navigate(routes.add());
    } catch (e) {
      console.error("Error adding a tree.", e);
      setError("Error adding tree, please try again later.");
    } finally {
      setBusy(false);
    }
  };

  const handleCancelClick = () => {
    console.debug("Tree adding cancelled, returning to home.");
    navigate(routes.home());
  };

  const handleNameChange = (value: string) => {
    setSpecies(value);
  };

  const handleStateChange = (value: string) => {
    setState(value);
  };

  return {
    canSave: isSaveEnabled(),
    circumference,
    diameter,
    error,
    handleCancelClick,
    handleCircumferenceChange,
    handleDiameterChange,
    handleHeightChange,
    handleNameChange,
    handleNotesChange,
    handleSaveClick,
    handleStateChange,
    height,
    notes,
    state,
  };
};
